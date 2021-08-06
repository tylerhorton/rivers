pub mod filter;
pub mod map;
pub mod to;

use crate::channel::{filter::Filter, map::Map, to::To};
use crate::event::Event;
use crate::transport::Transport;
use crate::Serializer;

pub trait Channel<'a, T: Transport>: Sized {
    type Key;
    type Value;

    fn next(&self) -> Event<Self::Key, Self::Value>;

    fn get_transport(&self) -> &'a T;

    fn map<K, V, F>(self, f: F) -> Map<'a, T, Self, F>
    where
        F: FnMut(Event<Self::Key, Self::Value>) -> Event<K, V>,
    {
        Map::new(self.get_transport(), self, f)
    }

    fn filter<P>(self, predicate: P) -> Filter<'a, T, Self, P>
    where
        P: FnMut(&Event<Self::Key, Self::Value>) -> bool,
    {
        Filter::new(self.get_transport(), self, predicate)
    }

    fn to<KS, VS>(
        self,
        topic: &str,
        key_serializer: KS,
        value_serializer: VS,
    ) -> To<'a, T, Self, KS, VS>
    where
        KS: Serializer<Self::Key>,
        VS: Serializer<Self::Value>,
    {
        To::new(
            self.get_transport(),
            self,
            topic.to_string(),
            key_serializer,
            value_serializer,
        )
    }
}
