//! You can treat this as a layout for future tests
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(resKern::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use resKern::print_ln;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    resKern::test_mode_panic_handling(_info)
}

#[test_case]
fn test_print_line() {
    print_ln!("This is a test");
}
