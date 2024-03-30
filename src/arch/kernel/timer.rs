use core::ptr;

use crate::lib::writel;
use crate::println;
use crate::{
    arch::arch_local_regs::{CNT_PNS_IRQ, TIMER_CNTRL0},
    arch::PBASE,
    kernel::timer::{HZ, NSEC_PER_SEC},
};

// System Timer on PI
const TIMER_CS: u32 = PBASE + 0x0000_3000;
const TIMER_CLO: u32 = PBASE + 0x0000_3004;
const TIMER_CHI: u32 = PBASE + 0x0000_3008;
const TIMER_C0: u32 = PBASE + 0x0000_300C;
const TIMER_C1: u32 = PBASE + 0x0000_3010;
const TIMER_C2: u32 = PBASE + 0x0000_3014;
const TIMER_C3: u32 = PBASE + 0x0000_3018;

const TIMER_CS_M0: u32 = 1 << 0;
const TIMER_CS_M1: u32 = 1 << 1;
const TIMER_CS_M2: u32 = 1 << 2;
const TIMER_CS_M3: u32 = 1 << 3;

// ARM side Timer on PI
// Reference: 12.2 Timer Registers
const ARM_TIMER_BASE: u32 = PBASE + 0xB000;
const ARM_TIMER_LOAD: u32 = ARM_TIMER_BASE + 0x400;
const ARM_TIMER_VALUE: u32 = ARM_TIMER_BASE + 0x404;
const ARM_TIMER_CTRL: u32 = ARM_TIMER_BASE + 0x408;
const ARM_TIMER_CLR: u32 = ARM_TIMER_BASE + 0x40C;

const CTRL_23BIT: u32 = 1 << 1; // 23-bit counter
const CTRL_INT_ENABLE: u32 = 1 << 5; // Timer interrupt enabled
const CTRL_ENABLE: u32 = 1 << 7; // Timer enabled

// Local timer
const PERIPHERAL_BASE: u32 = 0x3F00_0000; // Replace with the actual peripheral base address

const TIMER_CTRL: u32 = PERIPHERAL_BASE + 0x34;
const TIMER_FLAG: u32 = PERIPHERAL_BASE + 0x38;

const VAL: u64 = NSEC_PER_SEC / HZ;

fn generic_timer_init() -> i32 {
    unsafe {
        core::arch::asm!(
            "mov x0, #1",
            "msr cntp_ctl_el0, x0",
            options(nomem, nostack)
        );
    }
    0
}

fn generic_timer_reset(val: u64) -> i32 {
    unsafe {
        core::arch::asm!(
            "msr cntp_tval_el0, {timer_val:x}",
            timer_val = in(reg) val,
            options(nomem, nostack)
        );
    }
    0
}

fn enable_timer_interrupt() {
    writel(TIMER_CNTRL0, CNT_PNS_IRQ);
}

pub fn timer_init() {
    generic_timer_init();
    generic_timer_reset(VAL);
    enable_timer_interrupt();
}

pub fn handle_timer_irq() {
    generic_timer_reset(VAL);
    println!("Core0 Timer interrupt received");
}
