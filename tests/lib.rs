extern crate xmz_led_relais;

#[test]
fn led_works() {
    let leds = xmz_led_relais::LED::new();
    assert!(leds.data == 0);
}

#[test]
fn set_all() {
    let mut leds = xmz_led_relais::LED::new();
    // after creation all data is false, all led off
    assert!(leds.data == 0);

    leds.set_all();
    assert_eq!(leds.data, 0b11111111_11111111_11111111_11111111);
}


#[test]
fn set_one() {
    let mut leds = xmz_led_relais::LED::new();
    leds.set(1);
    assert_eq!(leds.data, 0b00000000_00000000_00000000_00000001);
}
