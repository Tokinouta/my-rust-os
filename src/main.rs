#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm_const)]

use core::panic::PanicInfo;

use crate::arch::kernel::{irq::arch_local_irq_enable, timer::timer_init};
use arch::sysregs;
use core::ffi::c_void;
use io::uart_init;

#[macro_use]
mod arch;
mod boot;
mod mm;
#[macro_use]
mod io;
mod kernel;
mod lib;

extern "C" {
    // static stext: *const c_void;
    // static etext: *const c_void;
    // static srodata: *const c_void;
    // static erodata: *const c_void;
    // static sdata: *const c_void;
    // static edata: *const c_void;
    // static sbss: *const c_void;
    // static ebss: *const c_void;
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
}

fn print_mem() {
    println!("MyRustOS image layout:");
    println!("    .text: 0x{:08x} - 0x{:08x}", stext as u64, etext as u64);
    println!(
        "  .rodata: 0x{:08x} - 0x{:08x}",
        srodata as u64, erodata as u64
    );
    println!("    .data: 0x{:08x} - 0x{:08x}", sdata as u64, edata as u64);
    println!("     .bss: 0x{:08x} - 0x{:08x}", sbss as u64, ebss as u64,);
}

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

    print_mem();
    timer_init();
    arch_local_irq_enable();

    panic!("you want to do nothing!");
}
