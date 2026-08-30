/*
 * AOS linker script.
 *
 * Hekate chainloads payloads at physical address 0x40008000 on the
 * Tegra X1 (this is the standard entry point used by every Switch
 * homebrew payload: Atmosphere, Lakka, hekate itself when chained, etc).
 * We link ourselves to run from that address so Hekate can jump straight
 * into _start with no relocation needed.
 */

ENTRY(_start)

MEMORY
{
  AOS_RAM (rwx) : ORIGIN = 0x40008000, LENGTH = 128M
}

SECTIONS
{
    .text : ALIGN(4096)
    {
        KEEP(*(.text._start))
        *(.text .text.*)
    } > AOS_RAM

    .rodata : ALIGN(4096)
    {
        *(.rodata .rodata.*)
    } > AOS_RAM

    .data : ALIGN(4096)
    {
        *(.data .data.*)
    } > AOS_RAM

    .bss : ALIGN(4096)
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } > AOS_RAM

    /* Stack grows down from the top of our claimed RAM region */
    . = ALIGN(16);
    . = . + 0x80000; /* 512KB stack */
    __stack_top = .;

    /DISCARD/ :
    {
        *(.comment)
        *(.eh_frame)
        *(.note.*)
    }
}
