#![no_std]
#![no_main]
// Disable these so we dont link the standard library and the default entry point is not rust level
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate lazy_static;
extern crate spin;
extern crate volatile;
extern crate x86_64;
use core::panic::PanicInfo;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_ln!("Hello Universe{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_ln!("{}", _info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    print_ln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExits::Success)
}

#[test_case]
fn oneisone() {
    print!("Asserting 1=1... ");
    assert_eq!(1, 1);
    print_ln!("[|OK|]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExits {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExits) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
