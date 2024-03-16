use core::arch::asm;

// ***************************************
// SCTLR_EL1, System Control Register (EL1), Page 2654 of
// AArch64-Reference-Manual.
// ***************************************
/* Common SCTLR_ELx flags. */
pub const SCTLR_ELx_DSSBS: u64 = 1 << 44;
pub const SCTLR_ELx_ENIA: u64 = 1 << 31;
pub const SCTLR_ELx_ENIB: u64 = 1 << 30;
pub const SCTLR_ELx_ENDA: u64 = 1 << 27;
pub const SCTLR_ELx_EE: u64 = 1 << 25;
pub const SCTLR_ELx_IESB: u64 = 1 << 21;
pub const SCTLR_ELx_WXN: u64 = 1 << 19;
pub const SCTLR_ELx_ENDB: u64 = 1 << 13;
pub const SCTLR_ELx_I: u64 = 1 << 12;
pub const SCTLR_ELx_SA: u64 = 1 << 3;
pub const SCTLR_ELx_C: u64 = 1 << 2;
pub const SCTLR_ELx_A: u64 = 1 << 1;
pub const SCTLR_ELx_M: u64 = 1 << 0;

pub const SCTLR_ELx_FLAGS: u64 =
    SCTLR_ELx_M | SCTLR_ELx_A | SCTLR_ELx_C | SCTLR_ELx_SA | SCTLR_ELx_I | SCTLR_ELx_IESB;

// SCTLR_EL2 specific flags
pub const SCTLR_EL2_RES1: u64 =
    1 << 4 | 1 << 5 | 1 << 11 | 1 << 16 | 1 << 18 | 1 << 22 | 1 << 23 | 1 << 28 | 1 << 29;
pub const SCTLR_EL2_RES0: u64 = 1 << 6
    | 1 << 7
    | 1 << 8
    | 1 << 9
    | 1 << 10
    | 1 << 13
    | 1 << 14
    | 1 << 15
    | 1 << 17
    | 1 << 20
    | 1 << 24
    | 1 << 26
    | 1 << 27
    | 1 << 30
    | 1 << 31
    | 0xffffefffu64 << 32;

#[cfg(target_endian = "big")]
pub const ENDIAN_SET_EL2: u64 = SCTLR_ELx_EE;
#[cfg(target_endian = "big")]
pub const ENDIAN_CLEAR_EL2: u64 = 0;

#[cfg(target_endian = "little")]
pub const ENDIAN_SET_EL2: u64 = 0;
#[cfg(target_endian = "little")]
pub const ENDIAN_CLEAR_EL2: u64 = SCTLR_ELx_EE;

/* SCTLR_EL2 value used for the hyp-stub */
pub const SCTLR_EL2_SET: u64 = SCTLR_ELx_IESB | ENDIAN_SET_EL2 | SCTLR_EL2_RES1;
pub const SCTLR_EL2_CLEAR: u64 = SCTLR_ELx_M
    | SCTLR_ELx_A
    | SCTLR_ELx_C
    | SCTLR_ELx_SA
    | SCTLR_ELx_I
    | SCTLR_ELx_WXN
    | SCTLR_ELx_DSSBS
    | ENDIAN_CLEAR_EL2
    | SCTLR_EL2_RES0;

pub const SCTLR_EL2_VALUE_MMU_DISABLED: u64 = SCTLR_EL2_RES1 | ENDIAN_SET_EL2;

// SCTLR_EL1 specific flags
pub const SCTLR_EL1_UCI: u64 = 1 << 26;
pub const SCTLR_EL1_E0E: u64 = 1 << 24;
pub const SCTLR_EL1_SPAN: u64 = 1 << 23;
pub const SCTLR_EL1_NTWE: u64 = 1 << 18;
pub const SCTLR_EL1_NTWI: u64 = 1 << 16;
pub const SCTLR_EL1_UCT: u64 = 1 << 15;
pub const SCTLR_EL1_DZE: u64 = 1 << 14;
pub const SCTLR_EL1_UMA: u64 = 1 << 9;
pub const SCTLR_EL1_SED: u64 = 1 << 8;
pub const SCTLR_EL1_ITD: u64 = 1 << 7;
pub const SCTLR_EL1_CP15BEN: u64 = 1 << 5;
pub const SCTLR_EL1_SA0: u64 = 1 << 4;

pub const SCTLR_EL1_RES1: u64 = 1 << 11 | 1 << 20 | 1 << 22 | 1 << 28 | 1 << 29;
pub const SCTLR_EL1_RES0: u64 =
    1 << 6 | 1 << 10 | 1 << 13 | 1 << 17 | 1 << 27 | 1 << 30 | 1 << 31 | 0xffffefffu64 << 32;

#[cfg(target_endian = "big")]
pub const ENDIAN_SET_EL1: u64 = SCTLR_EL1_E0E | SCTLR_ELx_EE;
#[cfg(target_endian = "big")]
pub const ENDIAN_CLEAR_EL1: u64 = 0;

#[cfg(target_endian = "little")]
pub const ENDIAN_SET_EL1: u64 = 0;
#[cfg(target_endian = "little")]
pub const ENDIAN_CLEAR_EL1: u64 = SCTLR_EL1_E0E | SCTLR_ELx_EE;

pub const SCTLR_EL1_SET: u64 = SCTLR_ELx_M
    | SCTLR_ELx_C
    | SCTLR_ELx_SA
    | SCTLR_EL1_SA0
    | SCTLR_EL1_SED
    | SCTLR_ELx_I
    | SCTLR_EL1_DZE
    | SCTLR_EL1_UCT
    | SCTLR_EL1_NTWE
    | SCTLR_ELx_IESB
    | SCTLR_EL1_SPAN
    | ENDIAN_SET_EL1
    | SCTLR_EL1_UCI
    | SCTLR_EL1_RES1;
pub const SCTLR_EL1_CLEAR: u64 = SCTLR_ELx_A
    | SCTLR_EL1_CP15BEN
    | SCTLR_EL1_ITD
    | SCTLR_EL1_UMA
    | SCTLR_ELx_WXN
    | ENDIAN_CLEAR_EL1
    | SCTLR_ELx_DSSBS
    | SCTLR_EL1_NTWI
    | SCTLR_EL1_RES0;

pub const SCTLR_EL1_VALUE_MMU_DISABLED: u64 = SCTLR_EL1_RES1 | ENDIAN_SET_EL1;

// ***************************************
// HCR_EL2, Hypervisor Configuration Register (EL2), Page 2487 of
// AArch64-Reference-Manual.
// ***************************************
pub const HCR_FWB: u64 = 1 << 46;
pub const HCR_API: u64 = 1 << 41;
pub const HCR_APK: u64 = 1 << 40;
pub const HCR_TEA: u64 = 1 << 37;
pub const HCR_TERR: u64 = 1 << 36;
pub const HCR_TLOR: u64 = 1 << 35;
pub const HCR_E2H: u64 = 1 << 34;
pub const HCR_ID: u64 = 1 << 33;
pub const HCR_CD: u64 = 1 << 32;
pub const HCR_RW_SHIFT: usize = 31;
pub const HCR_RW: u64 = 1 << HCR_RW_SHIFT;
pub const HCR_TRVM: u64 = 1 << 30;
pub const HCR_HCD: u64 = 1 << 29;
pub const HCR_TDZ: u64 = 1 << 28;
pub const HCR_TGE: u64 = 1 << 27;
pub const HCR_TVM: u64 = 1 << 26;
pub const HCR_TTLB: u64 = 1 << 25;
pub const HCR_TPU: u64 = 1 << 24;
pub const HCR_TPC: u64 = 1 << 23;
pub const HCR_TSW: u64 = 1 << 22;
pub const HCR_TAC: u64 = 1 << 21;
pub const HCR_TIDCP: u64 = 1 << 20;
pub const HCR_TSC: u64 = 1 << 19;
pub const HCR_TID3: u64 = 1 << 18;
pub const HCR_TID2: u64 = 1 << 17;
pub const HCR_TID1: u64 = 1 << 16;
pub const HCR_TID0: u64 = 1 << 15;
pub const HCR_TWE: u64 = 1 << 14;
pub const HCR_TWI: u64 = 1 << 13;
pub const HCR_DC: u64 = 1 << 12;
pub const HCR_BSU: u64 = 3 << 10;
pub const HCR_BSU_IS: u64 = 1 << 10;
pub const HCR_FB: u64 = 1 << 9;
pub const HCR_VSE: u64 = 1 << 8;
pub const HCR_VI: u64 = 1 << 7;
pub const HCR_VF: u64 = 1 << 6;
pub const HCR_AMO: u64 = 1 << 5;
pub const HCR_IMO: u64 = 1 << 4;
pub const HCR_FMO: u64 = 1 << 3;
pub const HCR_PTW: u64 = 1 << 2;
pub const HCR_SWIO: u64 = 1 << 1;
pub const HCR_VM: u64 = 1 << 0;

/*
 * The bits we set in HCR:
 * TLOR:	Trap LORegion register accesses
 * RW:		64bit by default, can be overridden for 32bit VMs
 * TAC:		Trap ACTLR
 * TSC:		Trap SMC
 * TVM:		Trap VM ops (until M+C set in SCTLR_EL1)
 * TSW:		Trap cache operations by set/way
 * TWE:		Trap WFE
 * TWI:		Trap WFI
 * TIDCP:	Trap L2CTLR/L2ECTLR
 * BSU_IS:	Upgrade barriers to the inner shareable domain
 * FB:		Force broadcast of all maintenance operations
 * AMO:		Override CPSR.A and enable signaling with VA
 * IMO:		Override CPSR.I and enable signaling with VI
 * FMO:		Override CPSR.F and enable signaling with VF
 * SWIO:	Turn set/way invalidates into set/way clean+invalidate
 */
pub const HCR_GUEST_FLAGS: u64 = HCR_TSC
    | HCR_TSW
    | HCR_TWE
    | HCR_TWI
    | HCR_VM
    | HCR_TVM
    | HCR_BSU_IS
    | HCR_FB
    | HCR_TAC
    | HCR_AMO
    | HCR_SWIO
    | HCR_TIDCP
    | HCR_RW
    | HCR_TLOR
    | HCR_FMO
    | HCR_IMO;
pub const HCR_VIRT_EXCP_MASK: u64 = HCR_VSE | HCR_VI | HCR_VF;
pub const HCR_HOST_NVHE_FLAGS: u64 = HCR_RW | HCR_API | HCR_APK;
pub const HCR_HOST_VHE_FLAGS: u64 = HCR_RW | HCR_TGE | HCR_E2H;

// ***************************************
// SCR_EL3, Secure Configuration Register (EL3), Page 2648 of
// AArch64-Reference-Manual.
// ***************************************
pub const SCR_RESERVED: u64 = 3 << 4;
pub const SCR_RW: u64 = 1 << 10;
pub const SCR_NS: u64 = 1 << 0;
pub const SCR_VALUE: u64 = SCR_RESERVED | SCR_RW | SCR_NS;

// ***************************************
// SPSR_EL3, Saved Program Status Register (EL3) Page 389 of
// AArch64-Reference-Manual.
// ***************************************
pub const SPSR_MASK_ALL: u64 = 7 << 6;
pub const SPSR_EL1h: u64 = 5 << 0;
pub const SPSR_EL2h: u64 = 9 << 0;
pub const SPSR_EL1: u64 = SPSR_MASK_ALL | SPSR_EL1h;
pub const SPSR_EL2: u64 = SPSR_MASK_ALL | SPSR_EL2h;

// Current Exception Level values, as contained in CurrentEL
pub const CurrentEL_EL1: u64 = 1 << 2;
pub const CurrentEL_EL2: u64 = 2 << 2;
pub const CurrentEL_EL3: u64 = 3 << 2;

macro_rules! read_sysreg {
    ($r:expr) => {{
        let mut _val: u64;
        unsafe {
            core::arch::asm!(format!("mrs {{}}, {}", $r),
            out(reg) _val, );
        }
        _val
    }};
}

macro_rules! write_sysreg {
    ($v:expr, $r:expr) => {{
        let _val = $v as u64;
        unsafe {
            core::arch::asm!(format!("msr {}, {{}}", $r),
            in(reg) _val);
        }
    }};
}

pub fn get_currentel() -> u64 {
    let mut _val: u64;
    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) _val);
    }
    _val >> 2
}
