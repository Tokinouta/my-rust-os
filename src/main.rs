#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::global_asm;
use core::panic::PanicInfo;

use uart::{uart_init, uart_recv, uart_send, uart_send_string};

mod uart;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("boot/boot.S"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart_init();
    uart_send_string("Welcome RararaOS!\r\n");
    uart_send_string("I am Gulaeer!\r\n");

    loop {
        uart_send(uart_recv());
    }
}
