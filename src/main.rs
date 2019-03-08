mod transceiver;
mod pin;
mod sonar;

extern crate sysfs_gpio;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use sonar::sonar::Sonar;
use transceiver::transceiver::Transceiver;
use sysfs_gpio::Pin;
use std::thread;
use std::time::Duration;
use crate::sonar::sonar_interface::SonarInterface;
use crate::transceiver::transceiver_interface::TransceiverInterface;


// cm/secs
pub const SOUND_SPEED: u64 = 34300;
//cm
const OBSTACLE_MIN_DISTANCE: u64 = 5;
const RED_LIGHT_PIN: u64 = 23;
const GREEN_LIGHT_PIN: u64 = 25;
const SHOCK_SENSOR_TRIGGER_PIN: u64 = 18;
const SHOCK_SENSOR_ECHO_PIN: u64 = 24;

fn manage_lights() {
    let red_light_pin = Pin::new(RED_LIGHT_PIN);
    let green_light_pin = Pin::new(GREEN_LIGHT_PIN);
    let shock_trigger_pin = Pin::new(SHOCK_SENSOR_TRIGGER_PIN);
    let shock_echo_pin = Pin::new(SHOCK_SENSOR_ECHO_PIN);

    let mut green_light_pin_value;
    let mut red_light_pin_value;

    let transceiver = Transceiver{};
    let sonar = Sonar{};

    let mut  distance;

    match sonar.distance_to_obstacle_cm(&shock_trigger_pin, &shock_echo_pin, &transceiver) {
        Ok(value) => distance = value,
        Err(_) => {
            println!("sonar failed to detect obstacle");
            return
        }
    }

    match distance <= OBSTACLE_MIN_DISTANCE {
        true => {
            green_light_pin_value = 0;
            red_light_pin_value = 1;
        }
        false => {
            green_light_pin_value = 1;
            red_light_pin_value = 0;
        }
    }

    transceiver.trigger(&red_light_pin, red_light_pin_value, 500);
}




fn main() {

    loop {
        manage_lights();
        thread::sleep(Duration::from_millis(250))
    }

}
