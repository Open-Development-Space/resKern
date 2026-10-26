use crate::{print, print_ln};
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
    print!("Initializing IDT... ");
    IDT.load();
    print_ln!("Loaded IDT!")
}

extern "x86-interrupt" fn bp_handler(stack_frame: InterruptStackFrame) {
    print_ln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
