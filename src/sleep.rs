

use core::arch::asm;

use crate::pi;
pub fn sleep(seconds: i32, pi: pi::Raspberrys) {
    let pi_model = pi::get_pi(pi);
    let c = pi_model.frequency/1000 * 1_000_000_000 * seconds;

    unsafe{
        for _ in 0..c {
            asm!("nop")
        }
    }
}