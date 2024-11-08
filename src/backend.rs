use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait PubSubBackend: Send + Sync {
    async fn publish(
        &self,
        channel: &str,
        message: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn subscribe(
        &self,
        channel: &str,
    ) -> Result<Box<dyn MessageStream + Send + Sync>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait MessageStream: Send + Sync {
    async fn next_message(&mut self) -> Option<String>;
}

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub struct InMemoryBackend {
    channels: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

struct InMemoryMessageStream {
    receiver: broadcast::Receiver<String>,
}

#[async_trait]
impl MessageStream for InMemoryMessageStream {
    async fn next_message(&mut self) -> Option<String> {
        match self.receiver.recv().await {
            Ok(msg) => Some(msg),
            Err(_) => None,
        }
    }
}

#[async_trait]
impl PubSubBackend for InMemoryBackend {
    async fn publish(
        &self,
        channel: &str,
        message: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let channels = self.channels.lock().await;
        if let Some(sender) = channels.get(channel) {
            sender.send(message.to_string())?;
            Ok(())
        } else {
            Err("Channel does not exist".into())
        }
    }

    async fn subscribe(
        &self,
        channel: &str,
    ) -> Result<Box<dyn MessageStream + Send + Sync>, Box<dyn Error + Send + Sync>> {
        let mut channels = self.channels.lock().await;
        let sender = channels
            .entry(channel.to_string())
            .or_insert_with(|| broadcast::channel(100).0);
        let receiver = sender.subscribe();
        Ok(Box::new(InMemoryMessageStream { receiver }))
    }
}

#[cfg(feature = "event-routing")]
mod redis_backend {
    use super::{MessageStream, PubSubBackend};
    use async_trait::async_trait;
    use futures_util::StreamExt;
    use redis::{AsyncCommands, Client, Msg, PubSub};
    use std::error::Error;
    use std::sync::Arc;

    pub struct RedisBackend {
        client: Client,
    }

    impl RedisBackend {
        pub fn new(client: Client) -> Self {
            Self { client }
        }
    }

    struct RedisMessageStream {
        pub_sub: PubSub,
    }

    #[async_trait]
    impl MessageStream for RedisMessageStream {
        async fn next_message(&mut self) -> Option<String> {
            self.pub_sub
                .on_message()
                .next()
                .await
                .and_then(|msg| msg.get_payload().ok())
        }
    }

    #[async_trait]
    impl PubSubBackend for RedisBackend {
        async fn publish(
            &self,
            channel: &str,
            message: &str,
        ) -> Result<(), Box<dyn Error + Send + Sync>> {
            let mut con = self.client.get_async_connection().await?;
            con.publish(channel, message).await?;
            Ok(())
        }

        async fn subscribe(
            &self,
            channel: &str,
        ) -> Result<Box<dyn MessageStream + Send + Sync>, Box<dyn Error + Send + Sync>> {
            let mut pub_sub = self.client.get_async_pubsub().await?;
            pub_sub.subscribe(channel).await?;
            Ok(Box::new(RedisMessageStream { pub_sub }))
        }
    }

    pub use RedisBackend;
}

#[cfg(feature = "event-routing")]
pub use redis_backend::RedisBackend;
