use super::transceiver_interface::TransceiverInterface;
use super::pin_interface::PinInterface;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

lazy_static! {
    static ref TRANSCEIVER_SPIES: Mutex<HashMap<String,Mutex<TransceiverSpy>>> =  Mutex::new(HashMap::new());
}

#[derive(Debug)]
struct TransceiverSpy {
    pub trigger_calls: usize,
    pub trigger_results: Vec<Result<(),Error>>,
    pub echo_calls: usize,
    pub echo_results: Vec<Result<u8, Error>>
}

impl TransceiverSpy {
    fn new() -> TransceiverSpy {
        return TransceiverSpy {
            trigger_calls: 0,
            trigger_results: Vec::new(),
            echo_calls: 0,
            echo_results: Vec::new()
        }
    }
}

#[derive(Clone,Debug)]
pub struct TransceiverMock {
    spy_id: String,
}

pub struct TransceiverMockGenerator {
}

impl TransceiverMockGenerator {
    pub fn generate_mock(&mut self) -> TransceiverMock {
        let id = generate_spy_id();
        let id_clone = id.clone();

        let spy = TransceiverSpy::new();
        let mock = TransceiverMock{spy_id: id_clone};

        store_spy(id, spy);
        return mock
    }
}

impl TransceiverInterface for TransceiverMock {
    fn trigger(&self, _pin: &impl PinInterface, _value: u8, _duration_ms: u64 ) -> Result<(), Error> {
        let spy_id = self.spy_id.as_str();
        let spy_map = TRANSCEIVER_SPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();
        spy.trigger_calls +=1;

        match spy.trigger_results[spy.trigger_calls -1 ] {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other,"error returned by mock")),
        }
    }

    fn echo(&self, _pin: &impl PinInterface, _timeout_ms: isize) -> Result<u8, Error> {
        let spy_id = self.spy_id.as_str();
        let spy_map = TRANSCEIVER_SPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();
        spy.echo_calls +=1;

        match spy.echo_results[spy.echo_calls -1 ] {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::new(ErrorKind::Other,"error returned by mock")),
        }
    }
}

impl TransceiverMock {
    pub fn when_trigger_called_return(&self, result: Result<(),Error>) {
        let spy_id = self.spy_id.as_str();
        let spy_map = TRANSCEIVER_SPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        spy.trigger_results.push(result);
    }

    pub fn verify_trigger_has_been_called(&self, calls: usize) -> bool {
        let spy_id = self.spy_id.as_str();
        let spy_map = TRANSCEIVER_SPIES.lock().unwrap();
        let spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        return spy.trigger_calls == calls
    }

    pub fn when_echo_called_return(&self, result: Result<u8, Error>) {
        let spy_id = self.spy_id.as_str();
        let spy_map = TRANSCEIVER_SPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        spy.echo_results.push(result);
    }

    pub fn verify_echo_has_been_called(&self, calls: usize) -> bool {
        let spy_id = self.spy_id.as_str();
        let spy_map = TRANSCEIVER_SPIES.lock().unwrap();
        let spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        return spy.echo_calls == calls
    }
}

fn store_spy(spy_id: String, spy: TransceiverSpy) {
    let mut spy_map =  TRANSCEIVER_SPIES.lock().unwrap();
    spy_map.insert(spy_id, Mutex::new(spy));
}

fn generate_spy_id() -> String {
    let mut rng = rand::thread_rng();
    let x =  rng.gen::<u32>();
    return x.to_string();
}

