use sysfs_gpio::{Direction, Pin};
use super::pin_interface::PinInterface;
use std::thread::sleep;
use std::time::Duration;
use std::io::Error;


impl PinInterface for Pin {
    fn transmit(&self, pin_num: u64, duration_ms: u64) -> Result<(), Error> {
        let pin = Pin::new(pin_num);
        pin.with_exported( || {
            pin.set_direction(Direction::Low);
            pin.set_value(1);
            sleep(Duration::from_millis(2000));
            Ok(())
        });
        Ok(())
    }
}