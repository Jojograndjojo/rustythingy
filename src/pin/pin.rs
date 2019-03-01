use sysfs_gpio::{Direction, Pin};
use super::pin_interface::PinInterface;
use std::thread::sleep;
use std::time::Duration;
use std::io::Error;


impl PinInterface for Pin {
    fn transmit(&self, value: u8, duration_ms: u64) -> Result<(), Error> {
        self.with_exported( || {
            self.set_direction(Direction::Low);
            self.set_value(value);
            sleep(Duration::from_millis(duration_ms));
            Ok(())
        });
        Ok(())
    }
}