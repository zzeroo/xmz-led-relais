extern crate xmz_led_relais;

#[test]
fn led_works() {
    let led = xmz_led_relais::LED::new();
    assert!(led.data.len() == 24);
}
