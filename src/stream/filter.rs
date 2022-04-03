use crate::event::{Event, FromEvent};
use crate::stream::Stream;

use async_trait::async_trait;
use futures::Future;
use std::marker::PhantomData;

pub struct Filter<St, F, T, Fut> {
    stream: St,
    f: F,
    _t_marker: PhantomData<fn() -> T>,
    _fut_marker: PhantomData<fn() -> Fut>,
}

impl<St, F, T, Fut> Filter<St, F, T, Fut> {
    pub(super) fn new(stream: St, f: F) -> Self {
        Filter {
            stream,
            f,
            _t_marker: PhantomData,
            _fut_marker: PhantomData,
        }
    }
}

#[async_trait]
impl<St, F, T, Fut> Stream for Filter<St, F, T, Fut>
where
    St: Stream,
    F: Fn(T) -> Fut + Send,
    T: FromEvent,
    Fut: Future<Output = bool> + Send,
{
    async fn next_event(&mut self) -> Option<Event> {
        while let Some(mut e) = self.stream.next_event().await {
            let res = (self.f)(T::from_event(&mut e));
            if res.await {
                return Some(e);
            }
        }

        None
    }
}
