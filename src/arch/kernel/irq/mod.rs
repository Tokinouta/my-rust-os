use core::{
    arch::{asm, global_asm},
    ptr,
};

use crate::{
    arch::{
        arch_local_regs::{ARM_LOCAL_IRQ_SOURCE0, CNT_PNS_IRQ},
        PBASE,
    },
    println,
};

use super::timer::handle_timer_irq;

global_asm!(
    include_str!("entry.S"),
    S_LR = const super::traps::S_LR,
    S_FRAME_SIZE = const super::traps::S_FRAME_SIZE,
    S_PC = const super::traps::S_PC,
);

pub const IRQ_BASIC_PENDING: u32 = PBASE + 0x0000B200;
pub const IRQ_PENDING_1: u32 = PBASE + 0x0000B204;
pub const IRQ_PENDING_2: u32 = PBASE + 0x0000B208;
pub const FIQ_CONTROL: u32 = PBASE + 0x0000B20C;
pub const ENABLE_IRQS_1: u32 = PBASE + 0x0000B210;
pub const ENABLE_IRQS_2: u32 = PBASE + 0x0000B214;
pub const ENABLE_BASIC_IRQS: u32 = PBASE + 0x0000B218;
pub const DISABLE_IRQS_1: u32 = PBASE + 0x0000B21C;
pub const DISABLE_IRQS_2: u32 = PBASE + 0x0000B220;
pub const DISABLE_BASIC_IRQS: u32 = PBASE + 0x0000B224;

pub const SYSTEM_TIMER_IRQ_0: u32 = 1 << 0;
pub const SYSTEM_TIMER_IRQ_1: u32 = 1 << 1;
pub const SYSTEM_TIMER_IRQ_2: u32 = 1 << 2;
pub const SYSTEM_TIMER_IRQ_3: u32 = 1 << 3;

pub const ARM_TIMER_IRQ: u32 = 1 << 0;

pub const PERIPHERAL_BASE: u32 = 0x3F00_0000; // Replace with the actual peripheral base address

pub const CORE0_INT_CTR: u32 = PERIPHERAL_BASE + 0x40;
pub const CORE0_INT_SOURCE: u32 = PERIPHERAL_BASE + 0x60;
pub const LOCAL_TIMER_INT: u32 = 1 << 11;
pub const CNTPNSIRQ_Int: u32 = 1 << 1;

#[inline(always)]
pub fn arch_local_irq_enable() {
    unsafe {
        asm!("msr daifclr, #2", options(nomem, nostack));
    }
}

#[inline(always)]
pub fn arch_local_irq_disable() {
    unsafe {
        asm!("msr daifset, #2", options(nomem, nostack));
    }
}

#[no_mangle]
pub fn irq_handle() {
    let irq: u32 = unsafe { ptr::read_volatile(ARM_LOCAL_IRQ_SOURCE0 as *const u32) };

    match irq {
        CNT_PNS_IRQ => handle_timer_irq(),
        _ => {
            println!("Unknown pending irq: {:x}", irq);
        }
    }
}
