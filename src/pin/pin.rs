use sysfs_gpio::{Direction, Pin};
use super::pin_interface::PinInterface;
use std::thread::sleep;
use std::time::Duration;
use std::io::Error;


impl PinInterface for Pin {
    fn transmit(&self, duration_ms: u64) -> Result<(), Error> {
        self.with_exported( || {
            self.set_direction(Direction::Low);
            self.set_value(1);
            sleep(Duration::from_millis(2000));
            Ok(())
        });
        Ok(())
    }
}