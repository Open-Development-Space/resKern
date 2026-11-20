use crate::arch::x86_64::{gdt, pits};
use crate::vga_buffer::Color::Red;
use crate::{print_cl, print_ln, print_ln_cl, vga_buffer::Color::Yellow};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(bp_handler);
        idt.divide_error.set_handler_fn(de_handler);
        idt.invalid_opcode.set_handler_fn(ud_handler);
        idt.general_protection_fault.set_handler_fn(gp_handler);
        idt.page_fault.set_handler_fn(pf_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(df_handler)
                .set_stack_index(gdt::DF_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(ts_handler);
        idt.segment_not_present.set_handler_fn(np_handler);
        idt.stack_segment_fault.set_handler_fn(ss_handler);
        idt.alignment_check.set_handler_fn(ac_handler);
        pits::init();
        idt[InterruptIndex::Timer.to_u8()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.to_u8()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    print_cl!(Yellow, "Initializing IDT... ");
    IDT.load();
    print_ln_cl!(Yellow, "Loaded IDT!")
}

/// Divide Error interrupts
extern "x86-interrupt" fn de_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR\n{:#?}\n", stack_frame);
}

/// Invalid Opcode
extern "x86-interrupt" fn ud_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: INVALID OPCODE\n{:#?}\n", stack_frame);
}

/// Bound Range Exceeded
extern "x86-interrupt" fn br_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}\n", stack_frame);
}

/// Invalid TSS
extern "x86-interrupt" fn ts_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: INVALID TSS: {}\n{:#?}\n",
        error_code, stack_frame
    );
}

/// Segment not present
extern "x86-interrupt" fn np_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: SEGMENT NOT PRESENT: {}\n{:#?}\n",
        error_code, stack_frame
    );
}

/// Stack Segment Fault
extern "x86-interrupt" fn ss_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: STACK SEGMENT FAULT: {}\n{:#?}\n",
        error_code, stack_frame
    );
}

/// Alignment Check
extern "x86-interrupt" fn ac_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: ALIGNMENT CHECK: {}\n{:#?}\n",
        error_code, stack_frame
    );
}

/// Handle breakpoint interrupts
extern "x86-interrupt" fn bp_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

/// General Protection Fault
extern "x86-interrupt" fn gp_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: GENERAL PROTECTION FAULT: {}\n{:#?}\n",
        error_code, stack_frame
    );
}

/// Handle double fault interrupts
extern "x86-interrupt" fn df_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT: {}\n{:#?}",
        error_code, stack_frame
    );
}

/// Handle timer interrupts
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    pits::update_TICKS();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.to_u8());
    }
}

/// Handle keyboard interrupts
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0x60);
    let scanned: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scanned);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.to_u8());
    }
}

/// Handle memory page interrupts
extern "x86-interrupt" fn pf_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    print_ln_cl!(Yellow, "A PAGE FAULT has occured");
    print_ln_cl!(Red, "Accessed Address: {:?}", Cr2::read());
    print_ln_cl!(Red, "Error Code: {:?}", error_code);
    print_ln_cl!(Red, "{:#?}", stack_frame);
    crate::halt_loop();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn to_u8(self) -> u8 {
        self as u8
    }
}
