extern crate rivers;
use rivers::{channel::Channel, event::Event, transport::Transport, Rivers};
use std::{cell::RefCell, thread, time};

struct TestTransport {
    events: Vec<(Vec<u8>, Vec<u8>)>,
    idx: RefCell<usize>,
}

impl TestTransport {
    pub fn new(events: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        TestTransport {
            events,
            idx: RefCell::new(0),
        }
    }
}

impl Transport for TestTransport {
    fn consume(&self, topic: &str) -> (Option<&[u8]>, Option<&[u8]>) {
        loop {
            thread::sleep(time::Duration::from_secs(1));

            let mut idx = self.idx.borrow_mut();
            if let Some((k, v)) = self.events.iter().nth(*idx) {
                println!(
                    "Consuming from topic {} with key {:?} and value {:?}",
                    topic, k, v
                );
                *idx += 1;
                return (Some(k.as_slice()), Some(v.as_slice()));
            } else {
                *idx = 0;
            }
        }
    }

    fn produce(&self, topic: &str, key: Option<Vec<u8>>, value: Option<Vec<u8>>) {
        println!(
            "Producing to topic {} with key {:?} and value {:?}",
            topic, key, value
        );
    }
}

fn bytes_to_u8(data: &[u8]) -> Result<u8, String> {
    if data.is_empty() {
        Err("No data for u8 deserializer".to_string())
    } else {
        Ok(data[0])
    }
}

fn u8_to_bytes(val: &u8) -> Vec<u8> {
    vec![*val]
}

fn u16_to_bytes(val: &u16) -> Vec<u8> {
    vec![(*val & 0x00FF) as u8, ((*val & 0xFF00) >> 8) as u8]
}

fn is_value_odd(e: &Event<u8, u8>) -> bool {
    if let Some(v) = e.value {
        v % 2 == 1
    } else {
        false
    }
}

fn square_value(e: Event<u8, u8>) -> Event<u8, u16> {
    if let Some(v) = e.value {
        Event::new(e.key, Some((v as u16).pow(2)))
    } else {
        Event::new(e.key, e.value.map(|v| v as u16))
    }
}

fn main() {
    let events = vec![(vec![1], vec![3]), (vec![2], vec![4])];
    let transport = TestTransport::new(events);
    let rivers = Rivers::new(transport);

    let stream = rivers
        .topic("foo", bytes_to_u8, bytes_to_u8)
        .filter(is_value_odd)
        .map(square_value)
        .to("bar", u8_to_bytes, u16_to_bytes);

    loop {
        stream.next();
    }
}
