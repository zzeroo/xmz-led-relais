extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};

use super::*;

#[derive(Debug,PartialEq)]
enum Bitorder {
    LSBFIRST,
    MSBFIRST,
}

pub fn shift_out(leds: LED) {
    let oe_pin = Pin::new(leds.oe_pin);
    let ds_pin = Pin::new(leds.ds_pin);
    let clock_pin = Pin::new(leds.clock_pin);
    let latch_pin = Pin::new(leds.latch_pin);
    let data = leds.data;

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
        Ok(()) => { let _ = oe_pin.set_value(0); }, // !OE pin low == Shift register enabled.
        Err(err) => println!("Could not set direction of DATA pin: {}", err),
    }

    match ds_pin.set_direction(Direction::Out) {
        Ok(()) => { let _ = ds_pin.set_value(0); },
        Err(err) => println!("Could not set direction of DATA pin: {}", err),
    }

    match clock_pin.set_direction(Direction::Out) {
        Ok(()) => { let _ = clock_pin.set_value(0); },
        Err(err) => println!("Could not set direction of CLOCK pin: {}", err),
    }

    match latch_pin.set_direction(Direction::Out) {
        Ok(()) => { let _ = latch_pin.set_value(0); },
        Err(err) => println!("Could not set direction of LATCH pin: {}", err),
    }


    // Clock in data
    let order: Bitorder = Bitorder::LSBFIRST;

    if order == Bitorder::MSBFIRST {
        for i in 0..23 {
            match (data >> i) & 1 {
                1 => { ds_pin.set_value(1); },
                _ => { ds_pin.set_value(0); },
            }
            let _ = clock_pin.set_value(1);
            let _ = clock_pin.set_value(0);
        }
    } else {
        for i in (0..23).rev() {
            match (data >> i) & 1 {
                1 => { ds_pin.set_value(1); },
                _ => { ds_pin.set_value(0); },
            }
            let _ = clock_pin.set_value(1);
            let _ = clock_pin.set_value(0);
        }
    }
    // Latch out
    let _ = latch_pin.set_value(1);
    let _ = latch_pin.set_value(0);

}
