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

impl_web! {
   impl Endpoint {
       ///@post("/send_msg")
       fn send_msg(&self) -> Result<Response<String>, http::Error> {

           Response::builder()
               .status(200)
               .body("just a string".to_string())
       }

       ///@post("/blink")
       fn blink(&self) -> Result<Response<String>, http::Error> {
           let led = Led::new(25);
           led.blink(10000, 250);

           Response::builder()
               .status(200)
               .body("green led should be blinking".to_string())
       }

       ///@post("/green_led_on")
       fn led_on(&self) -> Result<Response<String>, http::Error> {
           send_signal_to_pin(GREEN_LIGHT_PIN, 1, 2000)

           Response::builder()
               .status(200)
               .body("green led should be on".to_string())
       }

       ///@post("/red_led_on")
       fn new_led_on(&self) -> Result<Response<String>, http::Error> {
           send_signal_to_pin(RED_LIGHT_PIN, 1, 2000);

           Response::builder()
               .status(200)
               .body("red led should be on".to_string())
       }
   }
}

fn send_signal_to_pin(pin_num: u64, value: u8, duration_ms: u64) {
    let pin = Pin::new(pin_num);
    let transceiver = Transceiver{};
    transceiver.trigger(&pin, value, duration_ms);
}


fn main() {
    let addr = "0.0.0.0:8082".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(Endpoint)
        .run(&addr)
        .unwrap();

}
