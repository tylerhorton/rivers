mod filter;
mod map;

use crate::event::{Event, FromEvent};
use crate::stream::filter::Filter;
use crate::stream::map::Map;

use async_trait::async_trait;
use futures::Future;
use tokio::sync::broadcast::{error::RecvError, Receiver};

#[async_trait]
pub trait Stream: Sized + Send {
    async fn next<T: FromEvent>(&mut self) -> Option<T> {
        self.next_event().await.as_mut().map(T::from_event)
    }

    async fn next_event(&mut self) -> Option<Event>;

    fn filter<F, T, Fut>(self, f: F) -> Filter<Self, F, T, Fut>
    where
        F: Fn(T) -> Fut + Send,
        T: FromEvent,
        Fut: Future<Output = bool> + Send,
    {
        Filter::new(self, f)
    }

    fn map<F, In, Out, Fut>(self, f: F) -> Map<Self, F, In, Out, Fut> {
        Map::new(self, f)
    }
}

pub struct StreamSource {
    rx: Receiver<Event>,
}

impl StreamSource {
    pub fn new(rx: Receiver<Event>) -> Self {
        StreamSource { rx }
    }
}

#[async_trait]
impl Stream for StreamSource {
    async fn next_event(&mut self) -> Option<Event> {
        loop {
            match self.rx.recv().await {
                Ok(next) => return Some(next),
                Err(RecvError::Closed) => return None,
                Err(RecvError::Lagged(_)) => continue,
            }
        }
    }
}
