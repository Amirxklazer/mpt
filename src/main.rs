#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod uart;
mod fb;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart::init();
    uart::puts("AOS booting...\n");
    uart::puts("Milestone 1: alive.\n");

    fb::fill(0xFF00FF00); // solid green
    uart::puts("Milestone 2: framebuffer filled.\n");

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
