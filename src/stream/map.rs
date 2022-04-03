use crate::event::{Event, FromEvent, IntoEvent};
use crate::stream::Stream;

use async_trait::async_trait;
use futures::Future;
use std::marker::PhantomData;

pub struct Map<St, F, In, Out, Fut> {
    stream: St,
    f: F,
    _in_marker: PhantomData<fn() -> In>,
    _out_marker: PhantomData<fn() -> Out>,
    _fut_marker: PhantomData<fn() -> Fut>,
}

impl<St, F, In, Out, Fut> Map<St, F, In, Out, Fut> {
    pub(super) fn new(stream: St, f: F) -> Self {
        Map {
            stream,
            f,
            _in_marker: PhantomData,
            _out_marker: PhantomData,
            _fut_marker: PhantomData,
        }
    }
}

#[async_trait]
impl<St, F, In, Out, Fut> Stream for Map<St, F, In, Out, Fut>
where
    St: Stream,
    F: Fn(In) -> Fut + Send,
    In: FromEvent,
    Out: IntoEvent,
    Fut: Future<Output = Out> + Send,
{
    async fn next_event(&mut self) -> Option<Event> {
        while let Some(mut e) = self.stream.next_event().await {
            let out = (self.f)(In::from_event(&mut e));
            out.await.into_event(&mut e);
            return Some(e);
        }

        None
    }
}
