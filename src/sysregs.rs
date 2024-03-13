// ***************************************
// SCTLR_EL1, System Control Register (EL1), Page 2654 of
// AArch64-Reference-Manual.
// ***************************************
const SCTLR_RESERVED: u64 = (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11);
const SCTLR_EE_LITTLE_ENDIAN: u64 = 0 << 25;
const SCTLR_EOE_LITTLE_ENDIAN: u64 = 0 << 24;
const SCTLR_I_CACHE_DISABLED: u64 = 0 << 12;
const SCTLR_D_CACHE_DISABLED: u64 = 0 << 2;
const SCTLR_MMU_DISABLED: u64 = 0 << 0;
const SCTLR_MMU_ENABLED: u64 = 1 << 0;

const SCTLR_VALUE_MMU_DISABLED: u64 = SCTLR_RESERVED
    | SCTLR_EE_LITTLE_ENDIAN
    | SCTLR_I_CACHE_DISABLED
    | SCTLR_D_CACHE_DISABLED
    | SCTLR_MMU_DISABLED;

// ***************************************
// HCR_EL2, Hypervisor Configuration Register (EL2), Page 2487 of
// AArch64-Reference-Manual.
// ***************************************
const HCR_RW: u64 = 1 << 31;
const HCR_VALUE: u64 = HCR_RW;

// ***************************************
// SCR_EL3, Secure Configuration Register (EL3), Page 2648 of
// AArch64-Reference-Manual.
// ***************************************
const SCR_RESERVED: u64 = 3 << 4;
const SCR_RW: u64 = 1 << 10;
const SCR_NS: u64 = 1 << 0;
const SCR_VALUE: u64 = SCR_RESERVED | SCR_RW | SCR_NS;

// ***************************************
// SPSR_EL3, Saved Program Status Register (EL3) Page 389 of
// AArch64-Reference-Manual.
// ***************************************
const SPSR_MASK_ALL: u64 = 7 << 6;
const SPSR_EL1h: u64 = 5 << 0;
const SPSR_EL2h: u64 = 9 << 0;
const SPSR_EL1: u64 = SPSR_MASK_ALL | SPSR_EL1h;
const SPSR_EL2: u64 = SPSR_MASK_ALL | SPSR_EL2h;

// Current Exception Level values, as contained in CurrentEL
const CurrentEL_EL1: u64 = 1 << 2;
const CurrentEL_EL2: u64 = 2 << 2;
const CurrentEL_EL3: u64 = 3 << 2;
