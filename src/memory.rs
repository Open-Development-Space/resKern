use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
};

use crate::{print_ln_cl, vga_buffer::Color::Green};

/// Initialize new OffsetPageTable.
///
/// Caller must guarantee that the total physical memory space is mapped to virtual memory at the
/// phys_mem_offset. It must also only be called once.
pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    print_ln_cl!(Green, "Attempting to Initialize OffsetPageTable");
    unsafe {
        let l4_tab = active_L4_table(phys_mem_offset);
        OffsetPageTable::new(l4_tab, phys_mem_offset)
    }
}

/// Return a mut reference to the active L4 table.
///
/// The caller must guarantee that the total physical memory is mapped to virtual memory at the
/// given physical memory offset. It must also only be called once.
unsafe fn active_L4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    // pointer to the page table
    let ptab_ptr: *mut PageTable = virt.as_mut_ptr();

    // scary potentially undefined behaviour
    unsafe { &mut *ptab_ptr }
}

/// Translate a given VirtualAddr to a mapped PhysAddr or None if address is unmapped.
///
/// The caller mus guarantee that the complete physical memory is mapped to virutal memory at the
/// passed phys_mem_offset.
pub unsafe fn translate_addr(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_internal(addr, phys_mem_offset)
}

/// called by translate_addr.
fn translate_addr_internal(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (l4_table_frame, _) = Cr3::read();

    let table_indices = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = l4_table_frame;

    for &index in &table_indices {
        let virt = phys_mem_offset + frame.start_address().as_u64();
        let tab_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*tab_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages is not supported!"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

pub fn create_test_mapping(
    page: Page,
    mappr: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { mappr.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("map_to failed! :(").flush();
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        print_ln_cl!(Green, "Attempting to Initialize BootInfoFrameAllocator");
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
