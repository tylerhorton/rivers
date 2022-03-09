use crate::stream::StreamSource;

use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait Handler: Send + 'static {
    async fn call(self, s: StreamSource);
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: Fn(StreamSource) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(self, s: StreamSource) {
        self(s).await
    }
}
