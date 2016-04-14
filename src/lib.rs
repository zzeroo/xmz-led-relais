extern crate bit_vec;

use bit_vec::BitVec;

#[derive(PartialEq)]
pub struct LED {
    pub data: bit_vec::BitVec,
}

impl LED {
    pub fn new() -> LED {
        let data = BitVec::from_bytes(&[0u8, 0u8, 0u8]);
        LED { data: data, }
    }
}
