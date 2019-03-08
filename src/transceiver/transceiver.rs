use std::io::{Error, ErrorKind};
use super::pin_mock::PinMockGenerator;
use super::pin_interface::PinInterface;
use super::transceiver_interface::TransceiverInterface;


pub struct Transceiver {
}

impl TransceiverInterface for Transceiver {
    fn trigger(&self, pin: &impl PinInterface, value: u8, duration_ms: u64 ) -> Result<(), Error> {
        match pin.transmit(value, duration_ms) {
            Ok(()) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn echo(&self, pin: &impl PinInterface, timeout_ms: isize) -> Result<u8, Error> {
        match pin.read(timeout_ms) {
            Some(value) => Ok(value),
            None => Err(Error::new(ErrorKind::Other,"no value transmitted"))
        }
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
        let result = transceiver.trigger(&pin,10, 10);

        assert!(pin.verify_transmit_calls_has_been_called(1));
        assert!(!result.is_err())
    }

    #[test]
    fn should_poll_message() {
        let mut mock_generator = PinMockGenerator{};
        let pin = mock_generator.generate_mock();
        let pin_response = 1;
        pin.when_read_called_return(Some(pin_response));

        let transceiver = Transceiver{};
        let result = transceiver.echo(&pin, 100).unwrap();

        assert!(pin.verify_read_calls_has_been_called(1));
        assert_eq!(result, pin_response)
    }

}
