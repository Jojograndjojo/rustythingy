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
           let led = Led::new(25);
           led.switch_led_on();

           Response::builder()
               .status(200)
               .body("green led should be on".to_string())
       }

       ///@post("/red_led_on")
       fn new_led_on(&self) -> Result<Response<String>, http::Error> {
           let pin = Pin::new(23);
           let transceiver = Transceiver{};
           transceiver.trigger(&pin, 1, 2000);

           Response::builder()
               .status(200)
               .body("red led should be on".to_string())
       }
   }
}



fn main() {
    let addr = "0.0.0.0:8082".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(Endpoint)
        .run(&addr)
        .unwrap();

}
