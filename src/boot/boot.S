.section ".text.boot"

.globl _start
_start:
    mrs	x0, mpidr_el1
    and	x0, x0,#0xFF		// Check processor id
    cbz	x0, master		// Hang for all non-primary CPU
    b	proc_hang

proc_hang:
    b 	proc_hang

master:
    mrs x0, CurrentEL
    cmp x0, #{CurrentEL_EL3}
    b.eq el3_entry
    b el2_entry

el3_entry:
    ldr x0, ={SCTLR_EL2_VALUE_MMU_DISABLED}
    msr sctlr_el2, x0

    ldr x0, ={HCR_HOST_NVHE_FLAGS}
    msr hcr_el2, x0

    ldr x0, ={SCR_VALUE}
    msr scr_el3, x0

    ldr x0, ={SPSR_EL2}
    msr spsr_el3, x0

    adr x0, el2_entry
    msr elr_el3, x0

    eret

el2_entry:
    ldr x0, ={SCTLR_EL2_VALUE_MMU_DISABLED}
    msr sctlr_el2, x0

    ldr x0, ={SCTLR_EL1_VALUE_MMU_DISABLED}
    msr sctlr_el1, x0

    ldr x0, ={HCR_HOST_NVHE_FLAGS}
    msr hcr_el2, x0

    mov	x0, #0x33ff
    msr	cptr_el2, x0  // Disable coprocessor traps to EL2
    msr	hstr_el2, xzr // Disable coprocessor traps to EL2

    mov	x0, #3 << 20
    msr cpacr_el1, x0 // Enable FP/SIMD at EL1 

    ldr x0, ={SPSR_EL1}
    msr spsr_el2, x0

    adr x0, el1_entry
    msr elr_el2, x0

    eret

el1_entry:
    // adr x0, ebss
    // adr x1, sbss
    // sub x1, x1, x0
    // bl memzero

    ldr x0, =bootstacktop
    mov sp, x0
    bl rust_main
    b proc_hang // should never come here

.section .data
    .align 12
    .global bootstack
bootstack:
    .space 4096, 0
    .global bootstacktop
bootstacktop:
