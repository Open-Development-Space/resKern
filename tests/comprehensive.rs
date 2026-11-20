#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(resKern::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{BootInfo, entry_point};
use core::future::Future;
use core::panic::PanicInfo;
use core::pin::Pin;
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::{Context, Poll};
use resKern::BOOT_INFO;
use resKern::allocator;
use resKern::memory;
use resKern::memory::BootInfoFrameAllocator;
use resKern::task::Task;
use resKern::task::simple::SimpleExecutor;
use resKern::vga_buffer::Color;
use resKern::{print, print_cl, print_ln, print_ln_cl, sprint_ln};
use spin::Once;
use x86_64::VirtAddr;

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
    ensure_heap_ready();

    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    resKern::test_mode_panic_handling(_info)
}

#[test_case]
fn test_non_color_printing() {
    print!("This a test");
    print_ln!("This is also a test");
}

#[test_case]
fn test_color_printing() {
    for color in [
        Color::Black,
        Color::Blue,
        Color::Brown,
        Color::Cyan,
        Color::DarkGray,
        Color::Green,
        Color::LightBlue,
        Color::LightCyan,
        Color::LightGray,
        Color::LightGreen,
        Color::LightRed,
        Color::Magenta,
        Color::Pink,
        Color::Red,
        Color::White,
        Color::Yellow,
    ] {
        print_cl!(color, "█");
        print_ln_cl!(color, "█");
    }
}

#[test_case]
fn test_memory_heap_allocations() {
    let heap_stored_box = Box::new(41);

    sprint_ln!("Box heap memory value at {:p}", heap_stored_box);
}

#[test_case]
fn test_executor() {
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(do_a_thing()));
    executor.run();
}

#[test_case]
fn test_heap_vectors_are_distinct_and_accessible() {
    let mut values: Vec<u32> = Vec::new();
    for value in 0..512 {
        values.push(value);
    }

    assert_eq!(values.len(), 512);
    assert_eq!(values[0], 0);
    assert_eq!(values[511], 511);
}

#[test_case]
fn test_bump_allocator_alignment_and_reset() {
    use resKern::allocator::Locked;
    use resKern::allocator::bump_allocator::BumpAllocator;

    let bump = Locked::new(BumpAllocator::new());
    unsafe { bump.lock().init(0x10001, 128) };

    let layout = Layout::from_size_align(24, 16).unwrap();
    let first = unsafe { bump.alloc(layout) };
    assert!(!first.is_null());
    assert_eq!(first as usize % 16, 0);

    let second = unsafe { bump.alloc(layout) };
    assert!(!second.is_null());
    assert_ne!(first, second);
    assert_eq!(second as usize % 16, 0);

    unsafe {
        bump.dealloc(first, layout);
        bump.dealloc(second, layout);
    }

    let reused = unsafe { bump.alloc(layout) };
    assert_eq!(reused as usize, 0x10010);
    unsafe { bump.dealloc(reused, layout) };
}

#[test_case]
fn test_bump_allocator_rejects_oversized_allocations() {
    use resKern::allocator::Locked;
    use resKern::allocator::bump_allocator::BumpAllocator;

    let bump = Locked::new(BumpAllocator::new());
    unsafe { bump.lock().init(0x20000, 64) };

    let layout = Layout::from_size_align(65, 8).unwrap();
    let allocation = unsafe { bump.alloc(layout) };
    assert!(allocation.is_null());
}

static YIELD_TASK_COUNT: AtomicU32 = AtomicU32::new(0);

struct YieldOnce {
    yielded: bool,
}

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if self.yielded {
            YIELD_TASK_COUNT.fetch_add(1, Ordering::SeqCst);
            Poll::Ready(())
        } else {
            self.yielded = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[test_case]
fn test_executor_wakes_and_completes_tasks() {
    YIELD_TASK_COUNT.store(0, Ordering::SeqCst);

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(YieldOnce { yielded: false }));
    executor.spawn(Task::new(YieldOnce { yielded: false }));
    executor.spawn(Task::new(YieldOnce { yielded: false }));
    executor.run();

    assert_eq!(YIELD_TASK_COUNT.load(Ordering::SeqCst), 3);
}

async fn testable_value() -> u16 {
    4
}

async fn do_a_thing() {
    let value = testable_value().await;
    sprint_ln!("{}", value + value);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

#[test_case]
fn simple_allocation() {
    let x = Box::new(41);
    assert_eq!(*x, 41);
}

#[test_case]
fn many_boxes() {
    for i in 0..10000 {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn large_vec() {
    let mut vec = Vec::new();

    for i in 0..10000 {
        vec.push(i);
    }

    for i in 0..10000 {
        assert_eq!(vec[i], i);
    }
}

#[test_case]
fn huge_vec() {
    let vec = [0u8; 1024 * 1024];
    assert_eq!(vec.len(), 1024 * 1024);
}

#[test_case]
fn heap_write() {
    ensure_heap_ready();
    let mut value = Box::new(10);

    *value = 55;

    assert_eq!(*value, 55);
}

#[test_case]
fn stack_write() {
    let mut _value = 0u64;

    _value = 1234;

    assert_eq!(_value, 1234);
}

#[test_case]
fn enable_interrupts() {
    x86_64::instructions::interrupts::enable();

    assert!(x86_64::instructions::interrupts::are_enabled());
}

#[test_case]
fn timer_ticks() {
    let start = resKern::arch::x86_64::pits::ticks();

    for _ in 0..100000 {
        core::hint::spin_loop();
    }

    let end = resKern::arch::x86_64::pits::ticks();

    assert!(end > start);
}
