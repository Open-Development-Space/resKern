#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod key_interpreter;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};

#[cfg(test)]
entry_point!(test_start);

/// test entry point
#[cfg(test)]
fn test_start(boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    halt_loop();
}

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// init function
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    print_ln!("init successful");
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

    halt_loop();
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
