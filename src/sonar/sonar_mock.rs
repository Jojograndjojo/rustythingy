use super::transceiver_interface::TransceiverInterface;
use super::pin_interface::PinInterface;
use super::sonar_interface::SonarInterface;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

lazy_static! {
    static ref SONAR_SPIES: Mutex<HashMap<String,Mutex<SonarSpy>>> =  Mutex::new(HashMap::new());
}

#[derive(Debug)]
struct SonarSpy {
    pub distance_to_obstacle_cm_calls: usize,
    pub distance_to_obstacle_cm_results: Vec<Result<u64,Error>>,
}

impl SonarSpy {
    fn new() -> SonarSpy {
        return SonarSpy {
            distance_to_obstacle_cm_calls: 0,
            distance_to_obstacle_cm_results: Vec::new(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct SonarMock {
    spy_id: String,
}

pub struct SonarMockGenerator {
}

impl SonarMockGenerator {
    pub fn generate_mock(&mut self) -> SonarMock {
        let id = generate_spy_id();
        let id_clone = id.clone();

        let spy = SonarSpy::new();
        let mock = SonarMock{spy_id: id_clone};

        store_spy(id, spy);
        return mock
    }
}

impl SonarInterface for SonarMock {
    fn distance_to_obstacle_cm(
        &self,
        trigger_pin: &impl PinInterface,
        echo_pin: &impl PinInterface,
        transceiver: &impl TransceiverInterface,
    ) -> Result<u64, Error> {
        let spy_id = self.spy_id.as_str();
        let spy_map = SONAR_SPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();
        spy.trigger_calls +=1;

        match spy.trigger_results[spy.trigger_calls -1 ] {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::new(ErrorKind::Other,"error returned by mock")),
        }
    }
}

impl SonarMock {
    pub fn when_distance_to_obstacle_cm_called_return(&self, result: Result<u64,Error>) {
        let spy_id = self.spy_id.as_str();
        let spy_map = SONAR_SPIES.lock().unwrap();
        let mut spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        spy.distance_to_obstacle_cm_results.push(result);
    }

    pub fn verify_distance_to_obstacle_cm_has_been_called(&self, calls: usize) -> bool {
        let spy_id = self.spy_id.as_str();
        let spy_map = SONAR_SPIES.lock().unwrap();
        let spy = spy_map.get(spy_id).unwrap().lock().unwrap();

        return spy.distance_to_obstacle_cm_calls == calls
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

