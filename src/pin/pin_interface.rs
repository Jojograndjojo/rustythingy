use std::io::Error;

pub trait PinInterface {
    fn transmit(&self, pin_number: u64, duration_ms: u64) -> Result<(), Error>;
}
