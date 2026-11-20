use core::sync::atomic::AtomicU64;
use x86_64::instructions::port::Port;

static TICKS: AtomicU64 = AtomicU64::new(0);

/// initialize PITS for timer/clock and uptime
pub fn init() {
    let divisor: u16 = 11931; // 100 Hz

    unsafe {
        let mut command = Port::<u8>::new(0x43);
        let mut channel0 = Port::<u8>::new(0x40);

        // Channel 0 LSB then MSB Mode 3 square wave
        command.write(0x36);

        channel0.write((divisor & 0xFF) as u8);
        channel0.write((divisor >> 8) as u8);
    }
}

pub fn ticks() -> u64 {
    TICKS.load(core::sync::atomic::Ordering::Relaxed)
}

pub fn find_uptime_seconds() -> u64 {
    TICKS.load(core::sync::atomic::Ordering::Relaxed) / 100
}

pub fn update_TICKS() -> u64 {
    TICKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
}
