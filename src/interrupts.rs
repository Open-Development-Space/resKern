use crate::key_interpreter::ScanSetOne;
use crate::vga_buffer::Color::Red;
use crate::{gdt, print};
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
        unsafe {
            idt.double_fault
                .set_handler_fn(df_handler)
                .set_stack_index(gdt::DF_IST_INDEX);
        }
        idt[InterruptIndex::Timer.to_u8()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.to_u8()].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(pf_handler);
        idt
    };
}

pub fn init_idt() {
    print_cl!(Yellow, "Initializing IDT... ");
    IDT.load();
    print_ln_cl!(Yellow, "Loaded IDT!")
}

/// Handle breakpoint interrupts
extern "x86-interrupt" fn bp_handler(stack_frame: InterruptStackFrame) {
    print_ln!();
    print_ln_cl!(Yellow, "A BREAKPOINT has occured");
    panic!("EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

/// Handle double fault interrupts
extern "x86-interrupt" fn df_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    print_ln!();
    print_ln_cl!(Yellow, "A DOUBLE FAULT has occured");
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

/// Handle timer interrupts
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print_cl!(Yellow, ".");
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
    if let Some(scanned_key) = ScanSetOne::map_to_meaning(scanned) {
        if let Some(scanned_char) = ScanSetOne::meaning_to_char(&scanned_key) {
            print!("{}", scanned_char);
        }
    }

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
