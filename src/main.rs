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
               .body("units should be blinking".to_string())
       }

       ///@post("/led_on")
       fn led_on(&self) -> Result<Response<String>, http::Error> {
           let led = Led::new(25);
           led.switch_led_on();

           Response::builder()
               .status(200)
               .body("units should be on".to_string())
       }

       ///@post("/led_off")
       fn led_off(&self) -> Result<Response<String>, http::Error> {
           let led = Led::new(25);
           led.switch_led_off();

           Response::builder()
               .status(200)
               .body("units should be off".to_string())
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
