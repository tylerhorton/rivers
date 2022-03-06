use async_trait::async_trait;
use futures::Future;

use crate::event::FromEvent;
use crate::stream::StreamSource;

#[async_trait]
pub trait Handler<T>: Send + 'static {
    async fn call(self, chan: StreamSource<T>);
}

#[async_trait]
impl<T, F, Fut> Handler<T> for F
where
    T: FromEvent + Clone + Send + 'static,
    F: Fn(StreamSource<T>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(self, chan: StreamSource<T>) {
        self(chan).await
    }
}
