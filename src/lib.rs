#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

/// init function
pub fn init() {
    interrupts::init_idt();
}

/// Entry point for tests
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_mode_panic_handling(info)
}

/// Define a trait for use with tests
pub trait Testable {
    fn run(&self) -> ();
}

/// Implement a test run function
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        sprint_ln!("{}---\t", core::any::type_name::<T>());
        self();
        sprint_ln!("[|ok|]");
    }
}

/// run tests
pub fn test_runner(tests: &[&dyn Testable]) {
    sprint_ln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExits::Success)
}

pub fn test_mode_panic_handling(info: &PanicInfo) -> ! {
    sprint_ln!("[|Failed|]\n");
    sprint_ln!("Error: {}\n", info);
    exit_qemu(QemuExits::Failed);

    loop {}
}

/// Describe what exit codes exist
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExits {
    Success = 0x10,
    Failed = 0x11,
}

/// Exit Qemu with a given code
pub fn exit_qemu(exit_code: QemuExits) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
