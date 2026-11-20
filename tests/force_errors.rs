#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(resKern::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use bootloader::{BootInfo, entry_point};

use core::panic::PanicInfo;
use resKern::BOOT_INFO;
entry_point!(test_start);

fn test_start(boot_info: &'static BootInfo) -> ! {
    resKern::init();
    BOOT_INFO.call_once(|| boot_info);
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    resKern::test_mode_panic_handling(_info)
}

#[test_case]
fn force_bp() {
    x86_64::instructions::interrupts::int3();
}

#[test_case]
fn force_stack_overflow() {
    stack_overflow();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    core::hint::black_box(0);
}

#[test_case]
fn force_ud() {
    unsafe {
        core::arch::asm!("ud2");
    }
}

#[test_case]
fn force_de() {
    unsafe { core::arch::asm!("xor rdx, rdx", "mov rax, 1", "div rdx") }
}
