#![no_std]
#![no_main]
// Disable these so we dont link the standard library and the default entry point is not rust level
#![test_runner(resKern::test_runner)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate lazy_static;
extern crate spin;
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;
use core::panic::PanicInfo;

mod input_reader;
pub mod interrupts;
mod serial;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_ln!("Beginning init\n");

    resKern::init();

    unsafe {
        *(0xFFFFFFFFFFFFFFF as *mut u8) = 42;
    };

    #[cfg(test)]
    test_main();

    print_ln!("Did not crash");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_ln_cl!(vga_buffer::Color::Red, "{}", _info);
    loop {}
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
