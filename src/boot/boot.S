.section ".text.boot"

.globl _start
_start:
	mrs	x0, mpidr_el1
	and	x0, x0, #0xFF		// Check processor id
    ldr x1, =bootstacktop
    mov sp, x1
	cbz	x0, rust_main		// Hang for all non-primary CPU
	b	proc_hang

proc_hang:
	b 	proc_hang

.section .data
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop:
