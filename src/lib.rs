extern crate sysfs_gpio;

pub mod shift_register;


#[derive(Debug,PartialEq)]
pub struct LED {
    pub oe_pin: u64,
    pub ds_pin: u64,
    pub clock_pin: u64,
    pub latch_pin: u64,
    pub data: u32,
}

impl Default for LED {
    fn default() -> LED {
        LED { oe_pin: 276, ds_pin: 38, clock_pin: 44, latch_pin: 40, data: 0 }
    }
}

impl LED {
    pub fn new() -> LED {
        LED { ..Default::default() }
    }

    pub fn set(&mut self, led_num: usize) {
        self.data |= 1 << led_num - 1;
    }

    pub fn set_all(&mut self) {
        self.data = 0b11111111_11111111_11111111_11111111;
    }
}
