
use libm::roundf;
use core::arch::asm;

use crate::pi;
pub fn sleep(seconds: f32, pi: pi::Raspberrys) {
    let pi_model = pi::get_pi(pi);
    let c = roundf((pi_model.frequency as f32)/1000.0 * 1_000_000_000.0 * seconds) as i32;

    unsafe{
        for _ in 0..c {
            asm!("nop")
        }
    }
}