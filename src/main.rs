#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::global_asm;
use core::panic::PanicInfo;

use uart::uart_init;

#[macro_use]
mod io;
mod uart;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

global_asm!(include_str!("boot/boot.S"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart_init();

    extern "C" {
        fn _start();
        fn bootstacktop();
    }

    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    println!("Welcome RararaOS!");
    println!("I am Gulaeer!");
    panic!("you want to do nothing!");
}
