pub enum Raspberrys {
    Pi1,
    Pi2,
    Pi3,
    Pi3Plus,
}
    
pub struct Raspberry {
        pub wifi: bool,
        pub bluetooth: bool,
        pub ethernet: bool,
        pub cores: i32,
        pub frequency: i32,
}
    
pub const PI_1: Raspberry = Raspberry {
        wifi: false,
        bluetooth: false,
        ethernet: true,
        cores: 1,
        frequency: 700,
};

pub const PI_2: Raspberry = Raspberry {
        wifi: false,
        bluetooth: true,
        ethernet: true,
        cores: 4,
        frequency: 900,
};

pub const PI_3: Raspberry = Raspberry {
        wifi: true,
        bluetooth: true,
        ethernet: true,
        cores: 4,
        frequency: 1200,
};

pub const PI_3_PLUS: Raspberry = Raspberry {
        wifi: true,
        bluetooth: true,
        ethernet: true,
        cores: 4,
        frequency: 1400,
};


pub fn get_pi(raspberry: Raspberrys) -> Raspberry {
    match raspberry {
        Raspberrys::Pi1 => PI_1,
        Raspberrys::Pi2 => PI_2,
        Raspberrys::Pi3 => PI_3,
        Raspberrys::Pi3Plus => PI_3_PLUS,
    }
}