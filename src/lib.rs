pub mod bytes;
pub mod channel;
pub mod event;
pub mod transport;

use bytes::FromBytes;
use channel::Topic;
use transport::Transport;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
