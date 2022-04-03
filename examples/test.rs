extern crate rivers;

use rivers::adapters::{Key, Value};
use rivers::event::{Event, FromEvent, IntoEvent};
use rivers::stream::Stream;
use rivers::Rivers;

#[derive(Clone)]
struct U8Val(pub u8);
impl FromEvent for U8Val {
    fn from_event(e: &mut Event) -> Self {
        Self(*e.value.get(0).unwrap_or(&0))
    }
}
impl IntoEvent for U8Val {
    fn into_event(self, e: &mut Event) {
        e.value = vec![self.0]
    }
}

#[tokio::main]
async fn main() {
    let events = vec![
        Event::new(vec![1], vec![1]),
        Event::new(vec![1], vec![2]),
        Event::new(vec![3], vec![3]),
        Event::new(vec![3], vec![4]),
        Event::new(vec![5], vec![5]),
        Event::new(vec![5], vec![6]),
    ];

    let rivers = Rivers::new(events).stream("some topic", square_even_nums);

    rivers.run().await
}

async fn square_even_nums(s: impl Stream) {
    let is_even = |U8Val(v)| async move { v % 2 == 0 };
    let square = |U8Val(v)| async move { U8Val(v.pow(2)) };

    let mut new_s = s.filter(is_even).map(square);

    while let Some((Key(k), Value(v))) = new_s.next().await {
        println!("key: {:?}, value: {:?}", k, v);
    }
}
