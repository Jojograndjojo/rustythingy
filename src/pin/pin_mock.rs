use super::pin_interface::PinInterface;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

lazy_static! {
    static ref PINSPIES: Mutex<HashMap<String,Mutex<PinSpy>>> =  Mutex::new(HashMap::new());
}

#[derive(Debug)]
struct PinSpy {
    pub transmit_calls: usize,
    pub transmit_results: Vec<Result<(),Error>>,
    pub read_calls: usize,
    pub read_results: Vec<Option<u8>>
}

impl PinSpy {
    fn new() -> PinSpy {
        return PinSpy {
            transmit_calls: 0,
            transmit_results: Vec::new(),
            read_calls: 0,
            read_results: Vec::new()
        }
    }
}

#[derive(Clone,Debug)]
pub struct PinMock {
    spy_id: String,
}

pub struct PinMockGenerator {
}

impl PinMockGenerator {
    pub fn generate_mock(&mut self) -> PinMock {
        let id = generate_spy_id();
        let id_clone = id.clone();

        let spy = PinSpy::new();
        let pin_mock = PinMock{spy_id: id_clone};

        store_spy(id, spy);
        return pin_mock
    }
}

impl PinInterface for PinMock {
    fn transmit(&self, _value: u8, _duration_ms: u64) -> Result<(), Error> {
        let spy_id = self.spy_id.as_str();
        let spy_map = PINSPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();
        spy.transmit_calls +=1;

        match spy.transmit_results[spy.transmit_calls -1 ] {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other,"error returned by mock")),
        }
    }

    fn read(&self, _timeout_ms: isize) -> Option<u8> {
        let spy_id = self.spy_id.as_str();
        let spy_map = PINSPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();
        spy.read_calls +=1;

        return spy.read_results[spy.read_calls -1 ];
    }
}

impl PinMock {
    pub fn when_transmit_called_return(&self, result: Result<(),Error>) {
        let spy_id = self.spy_id.as_str();
        let spy_map = PINSPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();


        spy.transmit_results.push(result);
    }

    pub fn when_read_called_return(&self, result: Option<u8>) {
        let spy_id = self.spy_id.as_str();
        let spy_map = PINSPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        spy.read_results.push(result);
    }

    pub fn verify_transmit_calls_has_been_called(&self, calls: usize) -> bool {
        let spy_id = self.spy_id.as_str();
        let spy_map = PINSPIES.lock().unwrap();
        let spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        return spy.transmit_calls == calls
    }

    pub fn verify_read_calls_has_been_called(&self, calls: usize) -> bool {
        let spy_id = self.spy_id.as_str();
        let spy_map = PINSPIES.lock().unwrap();
        let spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        return spy.read_calls == calls
    }

}

fn store_spy(spy_id: String, spy: PinSpy) {
    let mut spy_map = PINSPIES.lock().unwrap();
    spy_map.insert(spy_id, Mutex::new(spy));
}


fn generate_spy_id() -> String {
    let mut rng = rand::thread_rng();
    let x =  rng.gen::<u32>();
    return x.to_string();
}

