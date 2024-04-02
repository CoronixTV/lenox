use crate::{gpio::{get_out_pin, OutPin, State}, pi, sleep};


pub struct SoftPwmPin {
    pin: OutPin,
    period: f32,
    duty_cycle: f32,
    running: bool,
}
fn get_soft_pwm_pin(pin: u32, period: f32, duty_cycle: f32) -> SoftPwmPin {
    SoftPwmPin {
        pin: get_out_pin(pin),
        period: period,
        duty_cycle: duty_cycle,
        running: false,
    }
}


impl SoftPwmPin {
    fn start(&mut self) {
        let high_time = self.period * self.duty_cycle;
        let low_time = self.period - high_time;
        self.running = true;
        loop {
            self.pin.set(State::High);
            sleep::sleep(high_time, pi::Raspberrys::Pi3Plus);
            self.pin.set(State::Low);
            sleep::sleep(low_time, pi::Raspberrys::Pi3Plus);
            if !self.running {
                break;
            }
        }
    }
    fn stop(&mut self) {
        self.pin.set(State::Low);
        self.running = false;
    }
}
