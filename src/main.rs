//! AOS - a from-scratch OS kernel for the Nintendo Switch.
//!
//! Milestone 1: boot via Hekate, take control of the CPU, and prove
//! it by printing over UART. No filesystem, no drivers beyond UART,
//! no scheduler, no GUI yet. Those come after this is confirmed
//! working on real hardware.

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod uart;

// Pull in the assembly boot stub so _start ends up in the binary.
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart::init();
    uart::puts("AOS booting...\n");
    uart::puts("Milestone 1: alive.\n");

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart::puts("PANIC: kernel halted.\n");
    loop {
        core::hint::spin_loop();
    }
}
