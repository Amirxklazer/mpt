//! Minimal framebuffer writer.
//!
//! Hekate configures the display and leaves a framebuffer active at a
//! fixed address before chainloading a payload — this is the same
//! address hekate's own boot screen code uses internally.

const FB_ADDRESS: usize = 0xC000_0000;

// Same size hekate itself clears for its own boot screen. Filling this
// whole span covers the visible screen regardless of exact row stride.
const FB_SIZE_WORDS: usize = 0x3C0000 / 4;

/// Fills the whole screen with one solid color.
pub fn fill(color: u32) {
    unsafe {
        let fb = FB_ADDRESS as *mut u32;
        for i in 0..FB_SIZE_WORDS {
            core::ptr::write_volatile(fb.add(i), color);
        }
    }
}
