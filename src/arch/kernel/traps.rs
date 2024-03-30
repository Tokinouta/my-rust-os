use core::mem;

use crate::println;

#[repr(C)]
pub struct PtRegs {
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
    pub orig_x0: u64,
    pub syscallno: u32,
    pub unused2: u32,
    pub orig_addr_limit: u64,
    pub unused: u64,
    pub stackframe: [u64; 2],
}

pub const S_FRAME_SIZE: usize = mem::size_of::<PtRegs>();
pub const S_X0: usize = mem::offset_of!(PtRegs, regs) + 0 * mem::size_of::<u64>();
pub const S_X1: usize = mem::offset_of!(PtRegs, regs) + 1 * mem::size_of::<u64>();
pub const S_X2: usize = mem::offset_of!(PtRegs, regs) + 2 * mem::size_of::<u64>();
pub const S_X3: usize = mem::offset_of!(PtRegs, regs) + 3 * mem::size_of::<u64>();
pub const S_X4: usize = mem::offset_of!(PtRegs, regs) + 4 * mem::size_of::<u64>();
pub const S_X5: usize = mem::offset_of!(PtRegs, regs) + 5 * mem::size_of::<u64>();
pub const S_X6: usize = mem::offset_of!(PtRegs, regs) + 6 * mem::size_of::<u64>();
pub const S_X7: usize = mem::offset_of!(PtRegs, regs) + 7 * mem::size_of::<u64>();
pub const S_X8: usize = mem::offset_of!(PtRegs, regs) + 8 * mem::size_of::<u64>();
pub const S_X10: usize = mem::offset_of!(PtRegs, regs) + 10 * mem::size_of::<u64>();
pub const S_X12: usize = mem::offset_of!(PtRegs, regs) + 12 * mem::size_of::<u64>();
pub const S_X14: usize = mem::offset_of!(PtRegs, regs) + 14 * mem::size_of::<u64>();
pub const S_X16: usize = mem::offset_of!(PtRegs, regs) + 16 * mem::size_of::<u64>();
pub const S_X18: usize = mem::offset_of!(PtRegs, regs) + 18 * mem::size_of::<u64>();
pub const S_X20: usize = mem::offset_of!(PtRegs, regs) + 20 * mem::size_of::<u64>();
pub const S_X22: usize = mem::offset_of!(PtRegs, regs) + 22 * mem::size_of::<u64>();
pub const S_X24: usize = mem::offset_of!(PtRegs, regs) + 24 * mem::size_of::<u64>();
pub const S_X26: usize = mem::offset_of!(PtRegs, regs) + 26 * mem::size_of::<u64>();
pub const S_X28: usize = mem::offset_of!(PtRegs, regs) + 28 * mem::size_of::<u64>();
pub const S_FP: usize = mem::offset_of!(PtRegs, regs) + 29 * mem::size_of::<u64>();
pub const S_LR: usize = mem::offset_of!(PtRegs, regs) + 30 * mem::size_of::<u64>();
pub const S_SP: usize = mem::offset_of!(PtRegs, sp);
pub const S_PSTATE: usize = mem::offset_of!(PtRegs, pstate);
pub const S_PC: usize = mem::offset_of!(PtRegs, pc);
pub const S_ORIG_X0: usize = mem::offset_of!(PtRegs, orig_x0);
pub const S_SYSCALLNO: usize = mem::offset_of!(PtRegs, syscallno);
pub const S_ORIG_ADDR_LIMIT: usize = mem::offset_of!(PtRegs, orig_addr_limit);
pub const S_STACKFRAME: usize = mem::offset_of!(PtRegs, stackframe);

const BAD_MODE_HANDLER: [&str; 4] = ["Sync Abort", "IRQ", "FIQ", "Error"];

#[no_mangle]
pub fn bad_mode(_regs: &mut PtRegs, reason: i32, esr: u32) {
    println!(
        "Bad mode for {} handler detected, esr=0x{:x}",
        BAD_MODE_HANDLER[reason as usize], esr
    );

    loop {}
}
