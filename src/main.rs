#![no_std]
#![no_main]
// Disable these so we dont link the standard library and the default entry point is not rust level
#![test_runner(resKern::test_runner)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;
extern crate lazy_static;
extern crate spin;
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;
mod key_interpreter;
use core::panic::PanicInfo;
pub mod gdt;
mod serial;
mod vga_buffer;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use resKern::memory::{BootInfoFrameAllocator, translate_addr};
use x86_64::structures::paging::Page;

use crate::vga_buffer::Color::Green;

entry_point!(kern_start);

fn kern_start(boot_info: &'static BootInfo) -> ! {
    use resKern::memory::{self, BootInfoFrameAllocator};
    use x86_64::{VirtAddr, structures::paging::Translate};

    print_ln!("Beginning kinit\n");
    resKern::init();

    print_ln_cl!(Green, "Configuring memory");

    let phys_mem_oset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_oset) };
    let mut fallocater = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let thing1 = Box::new(41);

    #[cfg(test)]
    test_main();

    print_ln!();
    print_ln!("finished kinit");
    print_ln!("This program resembles a kernel!");
    resKern::halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_ln_cl!(vga_buffer::Color::Red, "{}", _info);
    resKern::halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    resKern::test_mode_panic_handling(info)
}

/// Make a tautological assertion to test tests
#[test_case]
fn oneisone() {
    assert_eq!(1, 1);
}
