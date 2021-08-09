pub mod filter;
pub mod map;
pub mod to;

use crate::channel::{filter::Filter, map::Map, to::To};
use crate::event::Event;
use crate::serde::{serialize, Serializer};
use crate::transport::Transport;

pub trait Channel<'a, T: Transport + 'a>: Sized {
    type Key;
    type Value;

    fn next(&self) -> Event<Self::Key, Self::Value>;

    fn transport(&self) -> &'a T;

    fn produce<K, V, KS, VS>(
        &self,
        topic: &str,
        key: &Option<K>,
        value: &Option<V>,
        key_serializer: &KS,
        value_serializer: &VS,
    ) where
        KS: Serializer<K>,
        VS: Serializer<V>,
    {
        self.transport().produce(
            topic,
            serialize(key, key_serializer),
            serialize(value, value_serializer),
        )
    }

    fn map<K, V, F>(self, f: F) -> Map<'a, T, Self, F>
    where
        F: FnMut(Event<Self::Key, Self::Value>) -> Event<K, V>,
    {
        Map::new(self.transport(), self, f)
    }

    fn filter<P>(self, predicate: P) -> Filter<'a, T, Self, P>
    where
        P: FnMut(&Event<Self::Key, Self::Value>) -> bool,
    {
        Filter::new(self.transport(), self, predicate)
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
            self.transport(),
            self,
            topic.to_string(),
            key_serializer,
            value_serializer,
        )
    }
}
