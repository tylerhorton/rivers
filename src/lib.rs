pub mod channel;
pub mod event;
pub mod serde;
pub mod transport;

use channel::Channel;
use event::Event;
use serde::{deserialize, Deserializer};
use transport::Transport;

use std::marker::PhantomData;
use std::{thread, time};

pub struct Rivers<T: Transport> {
    transport: T,
}

impl<T: Transport> Rivers<T> {
    pub fn new(transport: T) -> Self {
        Rivers { transport }
    }

    pub fn run(&self) -> ! {
        loop {
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    pub fn topic<'a, K, V, KD: Deserializer<K>, VD: Deserializer<V>>(
        &'a self,
        topic: &str,
        key_deserializer: KD,
        value_deserializer: VD,
    ) -> Topic<'a, T, K, V, KD, VD> {
        Topic::new(
            &self.transport,
            topic.to_string(),
            key_deserializer,
            value_deserializer,
        )
    }
}

pub struct Topic<'a, T: Transport, K, V, KD: Deserializer<K>, VD: Deserializer<V>> {
    transport: &'a T,
    topic: String,
    key_deserializer: KD,
    value_deserializer: VD,
    _key_marker: PhantomData<*const K>,
    _val_marker: PhantomData<*const V>,
}

impl<'a, T: Transport, K, V, KD: Deserializer<K>, VD: Deserializer<V>> Topic<'a, T, K, V, KD, VD> {
    pub(crate) fn new(
        transport: &'a T,
        topic: String,
        key_deserializer: KD,
        value_deserializer: VD,
    ) -> Self {
        Topic {
            transport,
            topic,
            key_deserializer,
            value_deserializer,
            _key_marker: PhantomData,
            _val_marker: PhantomData,
        }
    }
}

impl<'a, T: Transport, K, V, KD: Deserializer<K>, VD: Deserializer<V>> Channel<'a, T>
    for Topic<'a, T, K, V, KD, VD>
{
    type Key = K;
    type Value = V;

    fn next(&self) -> Event<Self::Key, Self::Value> {
        let (k, v) = self.transport.consume(&self.topic);
        Event::new(
            deserialize(k, &self.key_deserializer),
            deserialize(v, &self.value_deserializer),
        )
    }

    fn transport(&self) -> &'a T {
        self.transport
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
