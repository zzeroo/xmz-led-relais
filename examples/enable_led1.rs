extern crate bit_vec;
extern crate xmz_led_relais;
extern crate sysfs_gpio;

use xmz_led_relais::{LED, shift_register};

fn main() {
    let mut leds = LED::new();
    leds.set(23);
    leds.set(21);

    shift_register::shift_out(leds)

}
