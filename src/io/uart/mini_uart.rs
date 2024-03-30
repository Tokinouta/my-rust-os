use crate::{io::uart::*, lib::{readl, writel}};
use core::ptr;

pub const AUX_ENABLES: u32 = PBASE + 0x0021_5004;
pub const AUX_MU_IO_REG: u32 = PBASE + 0x0021_5040;
pub const AUX_MU_IER_REG: u32 = PBASE + 0x0021_5044;
pub const AUX_MU_IIR_REG: u32 = PBASE + 0x0021_5048;
pub const AUX_MU_LCR_REG: u32 = PBASE + 0x0021_504C;
pub const AUX_MU_MCR_REG: u32 = PBASE + 0x0021_5050;
pub const AUX_MU_LSR_REG: u32 = PBASE + 0x0021_5054;
pub const AUX_MU_MSR_REG: u32 = PBASE + 0x0021_5058;
pub const AUX_MU_SCRATCH: u32 = PBASE + 0x0021_505C;
pub const AUX_MU_CNTL_REG: u32 = PBASE + 0x0021_5060;
pub const AUX_MU_STAT_REG: u32 = PBASE + 0x0021_5064;
pub const AUX_MU_BAUD_REG: u32 = PBASE + 0x0021_5068;

pub fn uart_send(c: u8) {
    while readl(AUX_MU_LSR_REG) & 0x20 == 0 {}
    writel(AUX_MU_IO_REG, c as u32);
}

pub fn uart_recv() -> u8 {
    while readl(AUX_MU_LSR_REG) & 0x01 == 0 {}
    readl(AUX_MU_IO_REG) as u8
}

pub fn uart_send_string(str: &str) {
    for c in str.chars() {
        uart_send(c as u8);
    }
}

fn dbg_puts(s: &str) {
    for c in s.chars() {
        while readl(AUX_MU_LSR_REG) & 0x20 == 0 {}
        writel(AUX_MU_IO_REG, c as u32);
    }
}

pub fn uart_init() {
    let mut selector = readl(GPFSEL1);
    selector &= !(7 << 12); // Clean gpio14
    selector |= 2 << 12; // Set alt5 for gpio14
    selector &= !(7 << 15); // Clean gpio15
    selector |= 2 << 15; // Set alt5 for gpio15
    unsafe { 
        writel(GPFSEL1, selector);
        
        writel(GPPUD, 0);
        delay(150);
        writel(GPPUDCLK0, (1 << 14) | (1 << 15));
        delay(150);
        writel(GPPUDCLK0, 0);

        // Enable mini uart (this also enables access to its registers)
        writel(AUX_ENABLES, 1);

        // Disable auto flow control and disable receiver and transmitter (for now)
        writel(AUX_MU_CNTL_REG, 0);

        // Disable receive and transmit interrupts
        writel(AUX_MU_IER_REG, 0);

        // Enable 8-bit mode
        writel(AUX_MU_LCR_REG, 3);

        // Set RTS line to be always high
        writel(AUX_MU_MCR_REG, 0);

        // Set baud rate to 115200
        writel(AUX_MU_BAUD_REG, 270);

        // Finally, enable transmitter and receiver
        writel(AUX_MU_CNTL_REG, 3);
    }
}