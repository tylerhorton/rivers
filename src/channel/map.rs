use crate::channel::Channel;
use crate::event::Event;
use crate::transport::Transport;

pub struct Map<'a, T: Transport, C, F> {
    transport: &'a T,
    channel: C,
    f: F,
}

impl<'a, T: Transport, C, F> Map<'a, T, C, F> {
    pub(crate) fn new(transport: &'a T, channel: C, f: F) -> Self {
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
