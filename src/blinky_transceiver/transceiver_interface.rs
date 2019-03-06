use std::io::Error;
use super::pin_interface::PinInterface;

pub trait TransceiverInterface {
    fn trigger(&self, pin: &impl PinInterface, value: u8, duration_ms: u64 ) -> Result<(), Error>;
    fn echo(&self, pin: &impl PinInterface, timeout_ms: isize) -> Result<u8, Error>;
}