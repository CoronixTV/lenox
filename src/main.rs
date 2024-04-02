#![no_std]
#![no_main]
mod gpio;
mod sleep;
mod pi;
use core::arch::asm;

use gpio::OutPin;
mod user;
 mod pwm;

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    user::main();
    loop {}
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}



