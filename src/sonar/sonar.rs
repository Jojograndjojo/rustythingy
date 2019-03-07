use super::sonar_interface::SonarInterface;
use super::pin_interface::PinInterface;
use super::pin_mock::PinMockGenerator;
use super::transceiver_interface::TransceiverInterface;
use super::transceiver_mock::TransceiverMockGenerator;
use super::SOUND_SPEED;
use std::io::{Error, ErrorKind};
use std::time::SystemTime;


pub struct Sonar {
}

impl SonarInterface for Sonar {
    fn distance_to_obstacle_cm(
        &self,trigger_pin:
        &impl PinInterface,
        echo_pin: &impl PinInterface,
        transceiver: &impl TransceiverInterface
    ) -> Result<u64, Error> {
        let sys_time = SystemTime::now();

        if transceiver.trigger(trigger_pin,1, 100).is_err() {
            return Err(Error::new(ErrorKind::Other, "trigger failed"))
        }

        if transceiver.echo(echo_pin,1000).is_err() {
            return Err(Error::new(ErrorKind::Other, "echo failed"))
        }

        match sys_time.duration_since(sys_time) {
            Ok(duration) => Ok((SOUND_SPEED * duration.as_secs())/2),
            Err(_) => Err(Error::new(ErrorKind::Other, "timing failed"))
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn should_calculate_distance_to_obstacle() {
        let mut pin_mock_generator = PinMockGenerator{};
        let trigger_pin = pin_mock_generator.generate_mock();
        let echo_pin = pin_mock_generator.generate_mock();
        let transceiver = TransceiverMockGenerator{}.generate_mock();
        let sonar = Sonar{};

        transceiver.when_trigger_called_return(Ok(()));
        transceiver.when_echo_called_return(Ok(1));

        let result = sonar.distance_to_obstacle_cm(&trigger_pin,&echo_pin,&transceiver);

        assert!(!result.is_err());
        assert!(transceiver.verify_trigger_has_been_called(1));
        assert!(transceiver.verify_echo_has_been_called(1));
    }
}