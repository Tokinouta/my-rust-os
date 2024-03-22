use crate::arch::PBASE;

pub const GPFSEL1: u32 = PBASE + 0x0020_0004;
pub const GPSET0: u32 = PBASE + 0x0020_001C;
pub const GPCLR0: u32 = PBASE + 0x0020_0028;
pub const GPPUD: u32 = PBASE + 0x0020_0094;
pub const GPPUDCLK0: u32 = PBASE + 0x0020_0098;

mod mini_uart;
mod pl_uart;

pub fn delay(mut n: u32) {
    while n > 0 {
        n -= 1;
    }
}

#[cfg(feature = "mini_uart")]
pub use mini_uart::{uart_init, uart_recv, uart_send, uart_send_string};
#[cfg(feature = "pl_uart")]
pub use pl_uart::{uart_init, uart_recv, uart_send, uart_send_string};
