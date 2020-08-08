#![no_std]
#![no_main]

extern crate rlibc;

use core::panic::PanicInfo;
mod vga_buffer;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}

}