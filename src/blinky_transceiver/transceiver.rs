use sysfs_gpio::Pin;
use super::pin_mock::{PinMockGenerator,PinMock};
use super::pin_interface::PinInterface;


pub struct Transceiver {
}

impl Transceiver {
    fn trigger(&self, pin: &impl PinInterface, pin_number: u64, duration_ms: u64 ) {
        pin.transmit(pin_number, duration_ms);
    }
}

mod test {
    use super::*;

    #[test]
    fn should_transmit_message() {
        let mut mock_generator = PinMockGenerator{};
        let pin = mock_generator.generate_mock();
        pin.when_transmit_called_return(Ok(()));

        let transceiver = Transceiver{};
        transceiver.trigger(&pin,1,1);

        assert!(pin.verify_transmit_calls_has_been_called(1))
    }

}
