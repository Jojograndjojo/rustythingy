mod units;
mod blinky_transceiver;
mod pin;

#[macro_use]
extern crate sysfs_gpio;
#[macro_use]
extern crate tower_web;
#[macro_use]
extern crate rand;
#[macro_use]
extern crate lazy_static;

use units::led::Led;
use blinky_transceiver::transceiver::Transceiver;

use tower_web::ServiceBuilder;
use http::Response;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::io::Error;

const RED_LIGHT_PIN: u64 = 23;
const GREEN_LIGHT_PIN: u64 = 25;
const SHOCK_SENSOR_TRIGGER_PIN: u64 = 18;
const SHOCK_SENSOR_ECHO_PIN: u64 = 24;


struct Endpoint;

fn send_signal_to_pin(pin_num: u64, value: u8, duration_ms: u64) {
    let pin = Pin::new(pin_num);
    let transceiver = Transceiver{};
    transceiver.trigger(&pin, value, duration_ms);
}


fn main() {

}
