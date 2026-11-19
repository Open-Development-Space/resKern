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
use core::panic::PanicInfo;
mod vga_buffer;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use resKern::{
    allocator,
    task::{Task, keyboard, simple::SimpleExecutor},
};

use crate::vga_buffer::Color::{Cyan, Green};

entry_point!(kern_start);

fn kern_start(boot_info: &'static BootInfo) -> ! {
    use resKern::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    print_ln!("Beginning kinit\n");
    resKern::init();

    print_ln_cl!(Green, "Configuring memory");

    let phys_mem_oset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_oset) };
    let mut fallocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    print_ln!("Attempting to allocate/init heap");
    allocator::heap_init(&mut mapper, &mut fallocator).expect("heap initialization failed");
    print_ln!("Making async executor");
    let mut executor = SimpleExecutor::new();
    print_ln!();
    print_ln!("finished kinit");
    print_ln!("This program resembles a kernel!");

    // Test/Showcase code. Actual kernel logic and loops to come later.
    print_ln!("Start of showcase");
    let hv = Box::new(41);
    print_ln_cl!(Cyan, "Heap stored value at {:p}", hv);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    print_ln_cl!(Cyan, "Vector values at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_ref = reference_counted.clone();
    print_ln_cl!(
        Cyan,
        "current ref count is {}",
        Rc::strong_count(&cloned_ref)
    );
    core::mem::drop(reference_counted);
    print_ln_cl!(
        Cyan,
        "reference count is now {}",
        Rc::strong_count(&cloned_ref)
    );

    print_ln!();
    executor.spawn(Task::new(do_an_async()));
    executor.run();
    print_ln!("End of showcase");
    // End of Test/Showcase code.
    print_ln!("Enabling Keyboard");
    executor.spawn(Task::new(keyboard::print_kp()));
    executor.run();

    #[cfg(test)]
    test_main();

    resKern::halt_loop();
}

async fn async_thing() -> u16 {
    4
}

async fn do_an_async() {
    let num = async_thing().await;
    print_ln_cl!(Cyan, "number from async function: {}", num);
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
