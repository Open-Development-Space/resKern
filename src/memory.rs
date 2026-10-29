use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{OffsetPageTable, PageTable},
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
