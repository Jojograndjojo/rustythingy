mod transceiver;
mod pin;
mod sonar;

extern crate sysfs_gpio;
extern crate rand;
#[macro_use]
extern crate lazy_static;

//use transceiver::transceiver::Transceiver;
//use sysfs_gpio::Pin;

//const RED_LIGHT_PIN: u64 = 23;
//const GREEN_LIGHT_PIN: u64 = 25;
//const SHOCK_SENSOR_TRIGGER_PIN: u64 = 18;
//const SHOCK_SENSOR_ECHO_PIN: u64 = 24;

// cm/secs
pub const SOUND_SPEED: u64 = 34300;

//fn send_signal_to_pin(pin_num: u64, value: u8, duration_ms: u64) {
//    let pin = Pin::new(pin_num);
//    let transceiver = Transceiver{};
//    transceiver.trigger(&pin, value, duration_ms);
//}


fn main() {

}
