use crate::channel::Channel;
use crate::event::Event;
use crate::transport::Transport;
use crate::Serializer;

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

    fn produce(&self, event: &Event<C::Key, C::Value>) {
        self.transport.produce(
            &self.topic,
            event.key.as_ref().map(&self.key_serializer),
            event.value.as_ref().map(&self.value_serializer),
        );
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
        self.produce(&e);
        e
    }

    fn get_transport(&self) -> &'a T {
        self.transport
    }
}
