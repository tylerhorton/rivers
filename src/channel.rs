pub mod filter;
pub mod map;
pub mod to;

use crate::bytes::ToBytes;
use crate::channel::{filter::Filter, map::Map, to::To};
use crate::event::Event;
use crate::transport::Transport;

pub trait Channel<'a, T: Transport> {
    type Key;
    type Value;

    fn next(&self) -> Event<Self::Key, Self::Value>;

    fn get_transport(&self) -> &'a T;

    fn map<K, V, F>(self, f: F) -> Map<'a, T, Self, F>
    where
        Self: Sized,
        F: FnMut(Event<Self::Key, Self::Value>) -> Event<K, V>,
    {
        Map::new(self.get_transport(), self, f)
    }

    fn filter<P>(self, predicate: P) -> Filter<'a, T, Self, P>
    where
        Self: Sized,
        P: FnMut(&Event<Self::Key, Self::Value>) -> bool,
    {
        Filter::new(self.get_transport(), self, predicate)
    }

    fn to(self, topic: &str) -> To<'a, T, Self>
    where
        Self: Sized,
        Self::Key: ToBytes,
        Self::Value: ToBytes,
    {
        To::new(self.get_transport(), self, topic.to_string())
    }
}
