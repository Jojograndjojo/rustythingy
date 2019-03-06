use std::io::Error;

pub trait PinInterface {
    fn transmit(&self, value: u8, duration_ms: u64) -> Result<(), Error>;
    fn read(&self, timeout_ms: isize) -> Option<u8>;
}
