extern crate xmz_led_relais;

#[test]
fn led_works() {
    let led = xmz_led_relais::LED::new();
    assert!(led.data.len() == 24);
}

#[test]
fn led_on_creation_all_false() {
    let led = xmz_led_relais::LED::new();
    for x in led.data.iter() {
        assert_eq!(x, false);
    }
}
