extern crate rivers;

use rivers::adapters::{Key, Value};
use rivers::event::Event;
use rivers::stream::{Stream, StreamSource};
use rivers::Rivers;

#[tokio::main]
async fn main() {
    let events = vec![
        Event::new(vec![0x1], vec![0x2]),
        Event::new(vec![0x3], vec![0x4]),
        Event::new(vec![0x5], vec![0x6]),
    ];

    let rivers = Rivers::new(events)
        .stream("key and value", key_and_value_handler)
        .stream("value only", value_handler);

    rivers.run().await
}

async fn value_handler(mut s: StreamSource<Value>) {
    while let Some(Value(v)) = s.next().await {
        println!("value: {:02X?}", v);
    }
}

async fn key_and_value_handler(mut s: StreamSource<(Key, Value)>) {
    while let Some((Key(k), Value(v))) = s.next().await {
        println!("key: {:02X?}, value: {:02X?}", k, v);
    }
}
