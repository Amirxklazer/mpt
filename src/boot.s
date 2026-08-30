// AOS boot stub.
// Hekate jumps here directly with the CPU in a mostly-unknown state
// (caches/MMU off, single core running). Our only jobs before Rust
// code can safely run:
//   1. Set up a stack pointer (Rust needs a valid stack immediately)
//   2. Zero the .bss section (Rust assumes zero-initialized statics)
//   3. Jump to kernel_main

.section .text._start
.global _start

_start:
    // Set stack pointer to the top of the region reserved in link.ld
    ldr     x0, =__stack_top
    mov     sp, x0

    // Zero .bss: __bss_start..__bss_end
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
1:
    cmp     x0, x1
    b.ge    2f
    str     xzr, [x0], #8
    b       1b
2:

    // Jump into Rust. Never expected to return, but if it does,
    // fall into an infinite loop rather than executing garbage.
    bl      kernel_main
hang:
    wfe
    b       hang
