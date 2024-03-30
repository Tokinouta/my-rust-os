use core::{arch::asm, ptr};

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn put32(addr: *mut u32, value: u32) {
    asm!(
        "str {0:w}, [{1}]",
        in(reg) value,
        in(reg) addr,
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn get32(addr: *const u32) -> u32 {
    let result: u32;
    asm!(
        "ldr {0:w}, [{1}]",
        lateout(reg) result,
        in(reg) addr,
    );
    result
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn delay(mut count: u32) {
    asm!(
        "1:
          subs {0:x}, {0:x}, #1
          bne 1b",
        inout(reg) count,
    );
}

#[inline(always)]
pub fn readl(addr: u32) -> u32 {
    unsafe { ptr::read_volatile(addr as *const u32) }
}

#[inline(always)]
pub fn writel(addr: u32, val: u32) {
    unsafe { ptr::write_volatile(addr as *mut u32, val) };
}