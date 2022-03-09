pub mod adapters;
pub mod event;
pub mod handler;
pub mod stream;

use futures::future;
use std::collections::HashMap;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::event::Event;
use crate::handler::Handler;
use crate::stream::StreamSource;

pub struct Rivers {
    events: Vec<Event>,
    topics: HashMap<String, Sender<Event>>,
    join_handles: Vec<JoinHandle<()>>,
}

impl Rivers {
    pub fn new(events: Vec<Event>) -> Self {
        Rivers {
            events,
            topics: HashMap::new(),
            join_handles: vec![],
        }
    }

    pub fn stream<S, H>(mut self, topic: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Handler,
    {
        let rx = self.get_channel_for_topic(topic);

        let handle = tokio::spawn(handler.call(StreamSource::new(rx)));

        self.join_handles.push(handle);
        self
    }

    pub async fn run(self) {
        for (_, sender) in self.topics {
            for e in &self.events {
                sender.send(e.clone()).unwrap();
            }
        }

        future::join_all(self.join_handles).await;
    }

    fn get_channel_for_topic(&mut self, topic: impl AsRef<str>) -> Receiver<Event> {
        if let Some(tx) = self.topics.get(topic.as_ref()) {
            return tx.subscribe();
        }

        let (tx, rx) = broadcast::channel(16);
        self.topics.insert(topic.as_ref().to_string(), tx);

        rx
    }
}
