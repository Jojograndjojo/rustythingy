use sysfs_gpio::Pin;


pub struct Transceiver {
}

impl Transceiver {

}

mod test {
    use super::*;

    #[test]
    fn should_transmit_message() {
        let pin_number = 1;
        let transceiver = Transceiver{};
        transceiver.trigger()
    }

}
