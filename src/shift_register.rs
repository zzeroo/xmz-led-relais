extern crate bit_vec;
extern crate sysfs_gpio;

use bit_vec::BitVec;
use sysfs_gpio::{Direction, Pin};


pub fn shift_out(oe_pin: u64, ds_pin: u64, clock_pin: u64, latch_pin: u64, data: bit_vec::BitVec) {
    let oe_pin = Pin::new(oe_pin);
    let ds_pin = Pin::new(ds_pin);
    let clock_pin = Pin::new(clock_pin);
    let latch_pin = Pin::new(latch_pin);
    let data = data;

    // try to export the needed pins, panic if it fails
    match oe_pin.export() {
        Ok(()) => (),
        Err(err) => println!("!OE (output enabled) pin could not be exported: {}", err),
    }
    match ds_pin.export() {
        Ok(()) => (),
        Err(err) => println!("DATA pin could not be exported: {}", err),
    }
    match clock_pin.export() {
        Ok(()) => (),
        Err(err) => println!("CLOCK pin could not be exported: {}", err),
    }
    match latch_pin.export() {
        Ok(()) => (),
        Err(err) => println!("LATCH pin could not be exported: {}", err),
    }

    // set pin directions
    match oe_pin.set_direction(Direction::Out) {
        Ok(()) => (), // !OE pin low == Shift register enabled.
        Err(err) => println!("Could not set direction of DATA pin: {}", err),
    }
    //oe_pin.set_value(0)
    match ds_pin.set_direction(Direction::Out) {
        Ok(()) => (),
        Err(err) => println!("Could not set direction of DATA pin: {}", err),
    }
    //ds_pin.set_value(0)
    match clock_pin.set_direction(Direction::Out) {
        Ok(()) => (),
        Err(err) => println!("Could not set direction of CLOCK pin: {}", err),
    }
    // clock_pin.set_value(0)
    match latch_pin.set_direction(Direction::Out) {
        Ok(()) => (),
        Err(err) => println!("Could not set direction of LATCH pin: {}", err),
    }
    // latch_pin.set_value(0)

    // Clock in data
    for x in data.iter() {
        ds_pin.set_value(x as u8);
        clock_pin.set_value(1);
        clock_pin.set_value(0);
    }
    // Latch out
    latch_pin.set_value(1);
    latch_pin.set_value(0);

}
