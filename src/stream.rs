mod next;

use crate::event::{Event, FromEvent};
use crate::stream::next::Next;

use futures::Stream as FuturesStream;
use pin_project::pin_project;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::broadcast::{error::RecvError, Receiver};
use tokio_util::sync::ReusableBoxFuture;

pub trait Stream: FuturesStream {
    fn next(&mut self) -> Next<'_, Self>
    where
        Self: Unpin,
    {
        Next::new(self)
    }

    fn poll_next_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
    where
        Self: Unpin,
    {
        Pin::new(self).poll_next(cx)
    }

    // fn map<F, T>(self, f: F) -> Map<Self, F>
    // where
    //     T: FromEvent,
    //     F: Fn(Self::Item) -> T,
    // {
    //     Map::new(self, f)
    // }

    // fn filter<P>(self, predicate: P) -> Filter<Self, P>
    // where
    //     P: Fn(&Self::Item) -> bool,
    // {
    //     Filter::new(self, predicate)
    // }
}

impl<T: ?Sized> Stream for T where T: FuturesStream {}

#[pin_project]
pub struct StreamSource<T> {
    chan: Receiver<Event>,
    _type_marker: PhantomData<fn() -> T>,
}
impl<T> StreamSource<T> {
    pub fn new(chan: Receiver<Event>) -> Self {
        StreamSource {
            chan,
            _type_marker: PhantomData,
        }
    }
}
impl<T> FuturesStream for StreamSource<T>
where
    T: FromEvent + Clone + Send + 'static,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let mut fut = ReusableBoxFuture::new(async { this.chan.recv().await });

        match futures::ready!(fut.poll(cx)) {
            Ok(mut next) => Poll::Ready(Some(T::from_event(&mut next))),
            Err(RecvError::Closed) => Poll::Ready(None),
            Err(RecvError::Lagged(_)) => Poll::Pending,
        }
    }
}

// pub struct Map<S, F> {
//     stream: S,
//     f: F,
// }
// impl<S, F> Map<S, F> {
//     fn new(stream: S, f: F) -> Self {
//         Map { stream, f }
//     }
// }
// impl<S, F, T> Stream for Map<S, F>
// where
//     S: Stream,
//     F: Fn(S::Item) -> T,
//     T: FromEvent,
// {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.stream.next().map(&self.f)
//     }
// }

// pub struct Filter<S, P> {
//     stream: S,
//     predicate: P,
// }
// impl<S, P> Filter<S, P> {
//     fn new(stream: S, predicate: P) -> Self {
//         Filter { stream, predicate }
//     }
// }
// impl<S, F> Stream for Filter<S, F>
// where
//     S: Stream,
//     F: Fn(&S::Item) -> bool,
// {
//     type Item = S::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(e) = self.stream.next() {
//             if (self.predicate)(&e) {
//                 return Some(e);
//             }
//         }

//         None
//     }
// }
