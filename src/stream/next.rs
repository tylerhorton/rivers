use crate::event::{Event, FromEvent};
use crate::stream::Stream;

use futures::Future;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Next<'a, St: ?Sized, T> {
    stream: &'a mut St,
    _marker: PhantomData<fn() -> T>,
}

impl<St: ?Sized + Unpin, T> Unpin for Next<'_, St, T> {}

impl<'a, St, T> Next<'a, St, T>
where
    St: ?Sized + Unpin + Stream,
{
    pub(super) fn new(stream: &'a mut St) -> Self {
        Self {
            stream,
            _marker: PhantomData,
        }
    }
}

impl<St, T> Future for Next<'_, St, T>
where
    St: ?Sized + Unpin + Stream<Item = Event>,
    T: FromEvent,
{
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.stream
            .poll_next_unpin(cx)
            .map(|opt_e| opt_e.map(|mut e| T::from_event(&mut e)))
    }
}
