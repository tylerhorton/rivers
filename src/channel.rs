use crate::bytes::{FromBytes, ToBytes};
use crate::event::Event;
use crate::transport::Transport;
use std::marker::PhantomData;

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

pub struct To<'a, T: Transport, C: Channel<'a, T>> {
    transport: &'a T,
    channel: C,
    topic: String,
}
impl<'a, T: Transport, C: Channel<'a, T>> To<'a, T, C>
where
    C::Key: ToBytes,
    C::Value: ToBytes,
{
    pub fn new(transport: &'a T, channel: C, topic: String) -> Self {
        To {
            transport,
            channel,
            topic,
        }
    }

    fn produce(&self, event: &Event<C::Key, C::Value>) {
        let key = if let Some(k) = &event.key {
            k.to_bytes()
        } else {
            vec![]
        };
        let value = if let Some(v) = &event.value {
            v.to_bytes()
        } else {
            vec![]
        };

        self.transport.produce(&self.topic, key, value);
    }
}
impl<'a, T: Transport, C: Channel<'a, T>> Channel<'a, T> for To<'a, T, C>
where
    C::Key: ToBytes,
    C::Value: ToBytes,
{
    type Key = C::Key;
    type Value = C::Value;

    fn next(&self) -> Event<Self::Key, Self::Value> {
        let e = self.channel.next();
        self.produce(&e);
        e
    }

    fn get_transport(&self) -> &'a T {
        self.transport
    }
}

pub struct Map<'a, T: Transport, C, F> {
    transport: &'a T,
    channel: C,
    f: F,
}
impl<'a, T: Transport, C, F> Map<'a, T, C, F> {
    fn new(transport: &'a T, channel: C, f: F) -> Self {
        Map {
            transport,
            channel,
            f,
        }
    }
}
impl<'a, T: Transport, C: Channel<'a, T>, K, V, F> Channel<'a, T> for Map<'a, T, C, F>
where
    F: Fn(Event<C::Key, C::Value>) -> Event<K, V>,
{
    type Key = K;
    type Value = V;

    fn next(&self) -> Event<K, V> {
        (self.f)(self.channel.next())
    }

    fn get_transport(&self) -> &'a T {
        self.transport
    }
}

pub struct Filter<'a, T: Transport, C, P> {
    transport: &'a T,
    channel: C,
    predicate: P,
}
impl<'a, T: Transport, C, P> Filter<'a, T, C, P> {
    fn new(transport: &'a T, channel: C, predicate: P) -> Self {
        Filter {
            transport,
            channel,
            predicate,
        }
    }
}
impl<'a, T: Transport, C: Channel<'a, T>, P> Channel<'a, T> for Filter<'a, T, C, P>
where
    P: Fn(&Event<C::Key, C::Value>) -> bool,
{
    type Key = C::Key;
    type Value = C::Value;

    fn next(&self) -> Event<C::Key, C::Value> {
        loop {
            let e = self.channel.next();
            if (self.predicate)(&e) {
                return e;
            }
        }
    }

    fn get_transport(&self) -> &'a T {
        self.transport
    }
}

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
