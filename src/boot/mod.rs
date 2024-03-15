use crate::mm;
use crate::sysregs;
use core::arch::global_asm;

global_asm!(
    include_str!("boot.S"),
    CurrentEL_EL3 = const sysregs::CurrentEL_EL3,
    SCTLR_VALUE_MMU_DISABLED = const sysregs::SCTLR_VALUE_MMU_DISABLED,
    HCR_VALUE = const sysregs::HCR_VALUE,
    SCR_VALUE = const sysregs::SCR_VALUE,
    SPSR_EL2 = const sysregs::SPSR_EL2,
    SPSR_EL1 = const sysregs::SPSR_EL1,
    // LOW_MEMORY = const mm::LOW_MEMORY,
);
