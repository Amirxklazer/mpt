//! Minimal driver for Tegra X1 UART-A.
//!
//! This is the same UART port every piece of Switch homebrew (hekate,
//! Atmosphere) uses for early debug logging, reachable over the console's
//! diagnostic pins with a serial-to-USB adapter. We don't need a full
//! driver yet — just enough to push bytes out so we have proof of life.
//!
//! Register offsets and reset values are from the Tegra X1 TRM. This
//! is polled (no interrupts, no FIFO management beyond checking
//! "transmit holding register empty") on purpose: at this stage we
//! want the simplest possible thing that can't fail silently.

const UART_A_BASE: usize = 0x7000_6000;

const UART_THR_DLAB0: usize = UART_A_BASE + 0x00; // Transmit Holding Register
const UART_LSR: usize = UART_A_BASE + 0x14; // Line Status Register

const LSR_THRE: u32 = 1 << 5; // Transmit Holding Register Empty

#[inline(always)]
unsafe fn mmio_read32(addr: usize) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

#[inline(always)]
unsafe fn mmio_write32(addr: usize, val: u32) {
    core::ptr::write_volatile(addr as *mut u32, val);
}

/// Blocks until the UART is ready to accept a byte, then writes it.
fn putc(c: u8) {
    unsafe {
        while mmio_read32(UART_LSR) & LSR_THRE == 0 {
            core::hint::spin_loop();
        }
        mmio_write32(UART_THR_DLAB0, c as u32);
    }
}

/// Writes a string to UART-A, translating '\n' to '\r\n' for terminal
/// compatibility.
pub fn puts(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            putc(b'\r');
        }
        putc(byte);
    }
}

/// Note: this assumes hekate has already configured UART-A's baud rate
/// and clock source, which it does before chainloading. A from-scratch
/// bringup (not booting via hekate) would need to program the clock
/// and divisor registers here first — that's a real gap, not an
/// oversight, and it's the first thing to fix if output stays silent
/// on hardware that didn't come through hekate.
pub fn init() {
    // Intentionally empty for now — see note above.
}
