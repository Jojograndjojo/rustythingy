use super::*;

pub struct Led {
    pin_number: u64,
}

impl Led {
    pub fn new(pin_number: u64) -> Led {
        return Led{pin_number}
    }

    pub fn blink(&self, duration_ms: u64, period_ms: u64) -> sysfs_gpio::Result<()> {
        let led = Pin::new(self.pin_number);
        led.with_exported(|| {
            led.set_direction(Direction::Low)?;
            let iterations = duration_ms / period_ms / 2;
            for _ in 0..iterations {
                led.set_value(0)?;
                sleep(Duration::from_millis(period_ms));
                led.set_value(1)?;
                sleep(Duration::from_millis(period_ms));
            }
            led.set_value(0)?;
            Ok(())
        })
    }

    pub fn switch_led_on(&self) {
        let led = Pin::new(self.pin_number);
        led.with_exported( || {
            led.set_direction(Direction::Low);
            led.set_value(1)
        });
    }

    pub fn switch_led_off(&self) {
        let led = Pin::new(self.pin_number);
        led.with_exported( || {
            led.set_direction(Direction::Low);
            led.set_value(0)
        });
    }
}



