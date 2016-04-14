extern crate bit_vec;
extern crate xmz_led_relais;
extern crate sysfs_gpio;

use xmz_led_relais::{LED, shift_register};

fn main() {
    let mut leds = LED::new();
    leds.set_all();

    shift_register::shift_out(leds.oe_pin, leds.ds_pin, leds.clock_pin, leds.latch_pin, leds.data)

}
