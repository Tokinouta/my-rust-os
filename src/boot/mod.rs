use crate::mm;
use crate::arch::sysregs;
use core::arch::global_asm;

global_asm!(
    include_str!("boot.S"),
    CurrentEL_EL3 = const sysregs::CurrentEL_EL3,
    SCTLR_EL2_VALUE_MMU_DISABLED = const sysregs::SCTLR_EL2_VALUE_MMU_DISABLED,
    SCTLR_EL1_VALUE_MMU_DISABLED = const sysregs::SCTLR_EL1_VALUE_MMU_DISABLED,
    HCR_HOST_NVHE_FLAGS = const sysregs::HCR_HOST_NVHE_FLAGS,
    SCR_VALUE = const sysregs::SCR_VALUE,
    SPSR_EL2 = const sysregs::SPSR_EL2,
    SPSR_EL1 = const sysregs::SPSR_EL1,
    // LOW_MEMORY = const mm::LOW_MEMORY,
);
