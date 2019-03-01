use std::io::Error;

pub trait PinInterface {
    fn transmit(&self, duration_ms: u64) -> Result<(), Error>;
}
