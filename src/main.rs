#![no_std]
#![no_main]
mod gpio;
mod sleep;
mod pi;
use core::arch::asm;

use gpio::OutPin;

 mod pwm;

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let pin = gpio::get_out_pin(16);
    loop {
        pin.set(gpio::State::High);
        sleep::sleep(1.0, pi::Raspberrys::Pi3Plus);
        pin.set(gpio::State::Low);
        sleep::sleep(1.0, pi::Raspberrys::Pi3Plus);
    }
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}



