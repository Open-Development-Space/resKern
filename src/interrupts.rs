use crate::{
    print_cl, print_ln, print_ln_cl,
    vga_buffer::Color::{Red, Yellow},
};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(bp_handler);
        idt
    };
}

pub fn init_idt() {
    print_cl!(Yellow, "Initializing IDT... ");
    IDT.load();
    print_ln_cl!(Yellow, "Loaded IDT!")
}

extern "x86-interrupt" fn bp_handler(stack_frame: InterruptStackFrame) {
    print_ln!();
    print_ln_cl!(Yellow, "A BREAKPOINT has occured");
    print_ln_cl!(Red, "EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

#[test_case]
fn test_bp_except() {
    x86_64::instructions::interrupts::int3();
}
