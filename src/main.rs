#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm_const)]

use core::panic::PanicInfo;

use arch::sysregs;
use io::uart_init;

#[macro_use]
mod arch;
mod boot;
mod mm;
#[macro_use]
mod io;
mod utils;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

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

    println!("Current EL: {}.", sysregs::get_currentel());

    panic!("you want to do nothing!");
}
