use crate::channel::Channel;
use crate::event::Event;
use crate::transport::Transport;

pub struct Filter<'a, T: Transport, C, P> {
    transport: &'a T,
    channel: C,
    predicate: P,
}

impl<'a, T: Transport, C, P> Filter<'a, T, C, P> {
    pub(crate) fn new(transport: &'a T, channel: C, predicate: P) -> Self {
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
