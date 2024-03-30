use crate::{io::uart::*, lib::{readl, writel}};
use core::ptr;

pub const U_BASE: u32 = PBASE + 0x00201000;
pub const U_DATA_REG: u32 = U_BASE;
pub const U_FR_REG: u32 = U_BASE + 0x18;
pub const U_IBRD_REG: u32 = U_BASE + 0x24;
pub const U_FBRD_REG: u32 = U_BASE + 0x28;
pub const U_LCRH_REG: u32 = U_BASE + 0x2C;
pub const U_CR_REG: u32 = U_BASE + 0x30;
pub const U_IMSC_REG: u32 = U_BASE + 0x38;

pub fn uart_send(c: u8) {
    // wait for transmit FIFO to have an available slot
    while readl(U_FR_REG) & 0x20 != 0 {}
    writel(U_DATA_REG, c as u32);
}

pub fn uart_recv() -> u8 {
    // wait for receive FIFO to have data to read
    while readl(U_FR_REG) & 0x10 != 0 {}
    readl(U_DATA_REG) as u8
}

pub fn uart_send_string(str: &str) {
    for c in str.chars() {
        uart_send(c as u8);
    }
}

pub fn uart_init() {
    let mut selector = readl(GPFSEL1);
    selector &= !(7 << 12); // Clean gpio14
    selector |= 4 << 12; // Set alt0 for gpio14
    selector &= !(7 << 15); // Clean gpio15
    selector |= 4 << 15; // Set alt0 for gpio15
    unsafe {
        writel(GPFSEL1, selector);

        writel(GPPUD, 0);
        delay(150);
        writel(GPPUDCLK0, (1 << 14) | (1 << 15));
        delay(150);
        writel(GPPUDCLK0, 0);

        // Disable UART until configuration is done
        writel(U_CR_REG, 0);

        /*
         * baud divisor = UARTCLK / (16 * baud_rate)
         * = 48 * 10^6 / (16 * 115200) = 26.0416666667
         * integer part = 26
         * fractional part = (int) ((0.0416666667 * 64) + 0.5) = 3
         * generated baud rate divisor = 26 + (3 / 64) = 26.046875
         * generated baud rate = (48 * 10^6) / (16 * 26.046875) = 115177
         * error = |(115177 - 115200) / 115200 * 100| = 0.02%
         */

        // Baud rate divisor, integer part
        writel(U_IBRD_REG, 26);

        // Baud rate divisor, fractional part
        writel(U_FBRD_REG, 3);

        // Enable FIFOs and 8-bit frames
        writel(U_LCRH_REG, (1 << 4) | (3 << 5));

        // Mask interrupts
        writel(U_IMSC_REG, 0);

        // Enable UART, receive, and transmit
        writel(U_CR_REG, 1 | (1 << 8) | (1 << 9));
    }
}
