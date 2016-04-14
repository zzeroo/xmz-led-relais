extern crate bit_vec;
extern crate sysfs_gpio;

use bit_vec::BitVec;

pub mod shift_register;


#[derive(Debug,PartialEq)]
pub struct LED {
    pub oe_pin: u64,
    pub ds_pin: u64,
    pub clock_pin: u64,
    pub latch_pin: u64,
    pub data: bit_vec::BitVec,
}

impl Default for LED {
    fn default() -> LED {
        let data = BitVec::from_bytes(&[0u8, 0u8, 0u8]);
        LED { oe_pin: 276, ds_pin: 38, clock_pin: 44, latch_pin: 40, data: data }
    }
}

impl LED {
    pub fn new() -> LED {
        LED { ..Default::default() }
    }

    pub fn set_all(&mut self) {
        self.data.set_all();
    }
}
