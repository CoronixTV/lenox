pub const GPIO_FSEL0: u32 = 0x3F20_0000;
pub const GPIO_FSEL1: u32 = 0x3F20_0004;
pub const GPIO_FSEL2: u32 = 0x3F20_0008;
pub const GPIO_SET0: u32 = 0x3F20_001C;
pub const GPIO_CLR0: u32 = 0x3F20_0028;
pub enum State {
    High,
    Low,
}


pub struct OutPin {
    pub pin: u32,
}
pub fn get_out_pin(pin: u32) -> OutPin {
    let reg = pin/10;
    let register = match reg {
        0 => GPIO_FSEL0,
        1 => GPIO_FSEL1,
        2 => GPIO_FSEL2,
        _ => panic!("Invalid pin number"),
    };

    let mut val: u32 = 0;
    unsafe {
        val = core::ptr::read_volatile(register as *mut u32);
    }
    let pinnum = pin%10;
    let mut mask:u32 = 0b111 << (pinnum*3);
    val &= !mask;
    val |= 1 << (pinnum*3);
    unsafe {
        core::ptr::write_volatile(register as *mut u32, val);
    }
    OutPin {
        pin: pin,
    }
}

impl OutPin {

    pub fn set(&self, state: State) {
        match state {
            State::High => {
                let bitpos = self.pin;
                let mut val: u32 = 0;
                unsafe {
                    val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
                }
                val |= 1 << bitpos;
                unsafe {
                    core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
                }
            }
            State::Low => {
                let bitpos = self.pin;
                let mut val: u32 = 0;
                unsafe {
                    val = core::ptr::read_volatile(GPIO_CLR0 as *mut u32);
                }
                val |= 1 << bitpos;
                unsafe {
                    core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val);
                }}
        }
    }
}


pub struct InPin {
    pub pin: u32,
}

impl InPin {
    fn read(&self) -> State{
        let reg = self.pin/10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Invalid pin number"),
        };
    
        let mut val: u32 = 0;
        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }
        if val & (1 << self.pin) != 0 {
            return State::High
        } else {
            return State::Low
        }}
}