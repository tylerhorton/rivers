use crate::bytes::ToBytes;
use crate::channel::Channel;
use crate::event::Event;
use crate::transport::Transport;

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
    pub(crate) fn new(transport: &'a T, channel: C, topic: String) -> Self {
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
