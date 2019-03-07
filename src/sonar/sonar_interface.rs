use std::io::Error;
use super::pin_interface::PinInterface;
use super::transceiver_interface::TransceiverInterface;

pub trait SonarInterface {
    fn distance_to_obstacle_cm(
        &self,
        trigger_pin: &impl PinInterface,
        echo_pin: &impl PinInterface,
        transceiver: &impl TransceiverInterface,
    ) -> Result<u64, Error>;
}