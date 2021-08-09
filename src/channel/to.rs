use crate::channel::Channel;
use crate::event::Event;
use crate::serde::Serializer;
use crate::transport::Transport;

pub struct To<'a, T: Transport, C: Channel<'a, T>, KS, VS>
where
    KS: Serializer<C::Key>,
    VS: Serializer<C::Value>,
{
    transport: &'a T,
    channel: C,
    topic: String,
    key_serializer: KS,
    value_serializer: VS,
}

impl<'a, T: Transport, C: Channel<'a, T>, KS, VS> To<'a, T, C, KS, VS>
where
    KS: Serializer<<C as Channel<'a, T>>::Key>,
    VS: Serializer<<C as Channel<'a, T>>::Value>,
{
    pub(crate) fn new(
        transport: &'a T,
        channel: C,
        topic: String,
        key_serializer: KS,
        value_serializer: VS,
    ) -> Self {
        To {
            transport,
            channel,
            topic,
            key_serializer,
            value_serializer,
        }
    }
}

impl<'a, T: Transport, C: Channel<'a, T>, KS, VS> Channel<'a, T> for To<'a, T, C, KS, VS>
where
    KS: Serializer<C::Key>,
    VS: Serializer<C::Value>,
{
    type Key = C::Key;
    type Value = C::Value;

    fn next(&self) -> Event<Self::Key, Self::Value> {
        let e = self.channel.next();
        self.produce(
            &self.topic,
            &e.key,
            &e.value,
            &self.key_serializer,
            &self.value_serializer,
        );
        e
    }

    fn transport(&self) -> &'a T {
        self.transport
    }
}
