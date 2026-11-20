#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(resKern::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use resKern::BOOT_INFO;
use resKern::allocator;
use resKern::arch::x86_64::pits;
use resKern::memory;
use resKern::memory::BootInfoFrameAllocator;
use resKern::sprint_ln;
use spin::Once;
use x86_64::VirtAddr;
use x86_64::structures::paging::FrameAllocator;

static HEAP_READY: Once<()> = Once::new();

fn ensure_heap_ready() {
    HEAP_READY.call_once(|| {
        let boot_info = BOOT_INFO.get().expect("BOOT_INFO not initialized");
        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { memory::init(phys_mem_offset) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

        allocator::heap_init(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");
    });
}

entry_point!(test_start);

fn test_start(boot_info: &'static BootInfo) -> ! {
    resKern::init();
    BOOT_INFO.call_once(|| boot_info);
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    resKern::test_mode_panic_handling(_info)
}

#[test_case]
fn frame_allocator_returns_unique_frames() {
    let boot_info = BOOT_INFO.get().unwrap();

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let frame1 = frame_allocator.allocate_frame().unwrap();
    let frame2 = frame_allocator.allocate_frame().unwrap();

    assert_ne!(frame1.start_address(), frame2.start_address());
}

#[test_case]
fn large_heap_usage() {
    ensure_heap_ready();

    let mut vec = Vec::new();

    for i in 0..100_000 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100_000);
}

#[test_case]
fn repeated_alloc_free() {
    ensure_heap_ready();

    for _ in 0..1000 {
        let x = Box::new(42);
        assert_eq!(*x, 42);
    }
}

#[test_case]
fn vec_integrity() {
    ensure_heap_ready();

    let mut vec = Vec::new();

    for i in 0..10000 {
        vec.push(i);
    }

    for i in 0..10000 {
        assert_eq!(vec[i], i);
    }
}

#[test_case]
fn multiple_live_allocations() {
    ensure_heap_ready();

    let mut objects = Vec::new();

    for i in 0..1000 {
        objects.push(Box::new(i));
    }

    for i in 0..1000 {
        assert_eq!(*objects[i], i);
    }
}

#[test_case]
fn timer_monotonically_increases() {
    let start = pits::ticks();

    for _ in 0..1_000_000 {
        core::hint::spin_loop();
    }

    let end = pits::ticks();

    assert!(end > start);
}

#[test_case]
fn validate_memory_map() {
    use bootloader::bootinfo::MemoryRegionType;

    let boot_info = BOOT_INFO.get().expect("BOOT_INFO not initialized");

    let mut usable_regions = 0;
    let mut usable_bytes = 0u64;

    for region in boot_info.memory_map.iter() {
        sprint_ln!(
            "{:?}: {:#018x} - {:#018x} ({} KiB)",
            region.region_type,
            region.range.start_addr(),
            region.range.end_addr(),
            (region.range.end_addr() - region.range.start_addr()) / 1024
        );

        // Basic sanity checks
        assert!(
            region.range.end_addr() > region.range.start_addr(),
            "memory region has invalid bounds"
        );

        // All regions should be page-aligned
        assert_eq!(
            region.range.start_addr() % 4096,
            0,
            "region start not page aligned"
        );

        assert_eq!(
            region.range.end_addr() % 4096,
            0,
            "region end not page aligned"
        );

        if region.region_type == MemoryRegionType::Usable {
            usable_regions += 1;
            usable_bytes += region.range.end_addr() - region.range.start_addr();
        }
    }

    sprint_ln!(
        "Found {} usable regions ({} MiB usable RAM)",
        usable_regions,
        usable_bytes / 1024 / 1024
    );

    assert!(usable_regions > 0);
    assert!(usable_bytes > 0);
}

#[test_case]
fn validate_memory_map_no_overlap() {
    let boot_info = BOOT_INFO.get().expect("BOOT_INFO not initialized");

    let regions: Vec<_> = boot_info.memory_map.iter().collect();

    for i in 0..regions.len() {
        for j in (i + 1)..regions.len() {
            let a_start = regions[i].range.start_addr();
            let a_end = regions[i].range.end_addr();

            let b_start = regions[j].range.start_addr();
            let b_end = regions[j].range.end_addr();

            let overlap = a_start < b_end && b_start < a_end;

            assert!(!overlap, "memory regions overlap");
        }
    }
}

#[test_case]
fn validate_frame_allocator() {
    let boot_info = BOOT_INFO.get().unwrap();

    let mut allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let mut last_addr = 0;

    for _ in 0..100 {
        let frame = allocator.allocate_frame().expect("out of frames");

        let addr = frame.start_address().as_u64();

        assert_eq!(addr % 4096, 0, "frame not page aligned");

        assert_ne!(addr, last_addr);

        last_addr = addr;
    }
}
