pub mod bytes;
pub mod channel;
pub mod event;
pub mod transport;

use bytes::FromBytes;
use channel::Channel;
use event::Event;
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

    pub fn topic<'a, K: FromBytes, V: FromBytes>(&'a self, topic: &str) -> Topic<'a, T, K, V> {
        Topic::new(&self.transport, topic.to_string())
    }
}

pub struct Topic<'a, T: Transport, K: FromBytes, V: FromBytes> {
    transport: &'a T,
    topic: String,
    _key_marker: PhantomData<*const K>,
    _val_marker: PhantomData<*const V>,
}

impl<'a, T: Transport, K: FromBytes, V: FromBytes> Topic<'a, T, K, V> {
    pub fn new(transport: &'a T, topic: String) -> Self {
        Topic {
            transport,
            topic,
            _key_marker: PhantomData,
            _val_marker: PhantomData,
        }
    }
}

impl<'a, T: Transport, K: FromBytes, V: FromBytes> Channel<'a, T> for Topic<'a, T, K, V> {
    type Key = K;
    type Value = V;

    fn next(&self) -> Event<Self::Key, Self::Value> {
        let (k, v) = self.transport.consume(&self.topic);
        let key = K::from_bytes(&k);
        let value = V::from_bytes(&v);
        Event { key, value }
    }

    fn get_transport(&self) -> &'a T {
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
