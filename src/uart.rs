const PBASE: u32 = 0x3F000000;

const GPFSEL1: u32 = PBASE + 0x0020_0004;
const GPSET0: u32 = PBASE + 0x0020_001C;
const GPCLR0: u32 = PBASE + 0x0020_0028;
const GPPUD: u32 = PBASE + 0x0020_0094;
const GPPUDCLK0: u32 = PBASE + 0x0020_0098;

const AUX_ENABLES: u32 = PBASE + 0x0021_5004;
const AUX_MU_IO_REG: u32 = PBASE + 0x0021_5040;
const AUX_MU_IER_REG: u32 = PBASE + 0x0021_5044;
const AUX_MU_IIR_REG: u32 = PBASE + 0x0021_5048;
const AUX_MU_LCR_REG: u32 = PBASE + 0x0021_504C;
const AUX_MU_MCR_REG: u32 = PBASE + 0x0021_5050;
const AUX_MU_LSR_REG: u32 = PBASE + 0x0021_5054;
const AUX_MU_MSR_REG: u32 = PBASE + 0x0021_5058;
const AUX_MU_SCRATCH: u32 = PBASE + 0x0021_505C;
const AUX_MU_CNTL_REG: u32 = PBASE + 0x0021_5060;
const AUX_MU_STAT_REG: u32 = PBASE + 0x0021_5064;
const AUX_MU_BAUD_REG: u32 = PBASE + 0x0021_5068;

const U_BASE: u32 = PBASE + 0x00201000;
const U_DATA_REG: u32 = U_BASE;
const U_FR_REG: u32 = U_BASE + 0x18;
const U_IBRD_REG: u32 = U_BASE + 0x24;
const U_FBRD_REG: u32 = U_BASE + 0x28;
const U_LCRH_REG: u32 = U_BASE + 0x2C;
const U_CR_REG: u32 = U_BASE + 0x30;
const U_IMSC_REG: u32 = U_BASE + 0x38;

use core::ptr;

fn delay(mut n: u32) {
    while n > 0 {
        n -= 1;
    }
}

pub fn uart_send(c: u8) {
    loop {
        if unsafe { ptr::read_volatile(AUX_MU_LSR_REG as *const u32) } & 0x20 != 0 {
            break;
        }
    }
    unsafe { ptr::write_volatile(AUX_MU_IO_REG as *mut u32, c as u32) };
}

pub fn uart_recv() -> u8 {
    loop {
        if unsafe { ptr::read_volatile(AUX_MU_LSR_REG as *const u32) } & 0x01 != 0 {
            break;
        }
    }
    unsafe { ptr::read_volatile(AUX_MU_IO_REG as *const u32) as u8 }
}

pub fn uart_send_string(str: &str) {
    for c in str.chars() {
        uart_send(c as u8);
    }
}

fn dbg_puts(s: &str) {
    for c in s.chars() {
        while unsafe { ptr::read_volatile(AUX_MU_LSR_REG as *const u32) } & 0x20 == 0 {}
        unsafe { ptr::write_volatile(AUX_MU_IO_REG as *mut u32, c as u32) };
    }
}

pub fn uart_init() {
    let selector = unsafe { ptr::read_volatile(GPFSEL1 as *const u32) };
    let mut selector = selector & !(7 << 12); /* clean gpio14 */
    selector |= 2 << 12; /* set alt5 for gpio14 */
    selector &= !(7 << 15); /* clean gpio15 */
    selector |= 2 << 15; /* set alt5 for gpio15 */
    unsafe { ptr::write_volatile(GPFSEL1 as *mut u32, selector) };

    unsafe {
        ptr::write_volatile(GPPUD as *mut u32, 0);
        delay(150);
        ptr::write_volatile(GPPUDCLK0 as *mut u32, (1 << 14) | (1 << 15));
        delay(150);
        ptr::write_volatile(GPPUDCLK0 as *mut u32, 0);

        /* Enable mini uart (this also enables access to it registers) */
        ptr::write_volatile(AUX_ENABLES as *mut u32, 1);

        /* Disable auto flow control and disable receiver and transmitter (for now) */
        ptr::write_volatile(AUX_MU_CNTL_REG as *mut u32, 0);

        /* Disable receive and transmit interrupts */
        ptr::write_volatile(AUX_MU_IER_REG as *mut u32, 0);

        /* Enable 8 bit mode */
        ptr::write_volatile(AUX_MU_LCR_REG as *mut u32, 3);

        /* Set RTS line to be always high */
        ptr::write_volatile(AUX_MU_MCR_REG as *mut u32, 0);

        /* Set baud rate to 115200 */
        ptr::write_volatile(AUX_MU_BAUD_REG as *mut u32, 270);

        /* Finally, enable transmitter and receiver */
        ptr::write_volatile(AUX_MU_CNTL_REG as *mut u32, 3);
    }
}
