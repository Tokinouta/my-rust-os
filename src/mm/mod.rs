use core::arch::asm;

const PAGE_SHIFT: u32 = 12;
const TABLE_SHIFT: u32 = 9;
const SECTION_SHIFT: u32 = PAGE_SHIFT + TABLE_SHIFT;

const PAGE_SIZE: u32 = 1 << PAGE_SHIFT;
const SECTION_SIZE: u32 = 1 << SECTION_SHIFT;

const LOW_MEMORY: u16 = 2 * SECTION_SIZE as u16;

#[inline(always)]
#[no_mangle]
pub fn memzero(mut src: *mut u64, mut n: usize) {
    unsafe {
        asm!(
            "1:
              str xzr, [{0}], #8
              subs {1}, {1}, #8
              b.gt 1b",
            inout(reg) src,
            inout(reg) n,
            // No clobbers since xzr and n are used as input and output operands
        );
    }
}

