#![no_std]
#![no_main]

mod run;
mod panic_handler;
#[macro_use]
pub mod utils;
pub mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    run::run();
    panic!("OS terminated")
}
