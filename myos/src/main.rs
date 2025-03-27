#![no_std]
#![no_main]


#[macro_use]
mod console;
mod sbi;
use core::arch::global_asm;
mod lang_items;


global_asm!(include_str!("entry.asm"));

// os/src/main.rs
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello,world");
    panic!("Shutdown machine");
}

pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}