#![no_std]
#![no_main]
// Disable these so we dont link the standard library and the default entry point is not rust level

extern crate lazy_static;
extern crate spin;
extern crate volatile;
use core::panic::PanicInfo;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_ln!("Hello Universe{}", "!");
    panic!("No more viable information");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_ln!("{}", _info);
    loop {}
}
