use crate::backend::PubSubBackend;
use crate::errors::AuroraWatchError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::{self, Sender as WatchSender};
use tokio::sync::Mutex;

/// Struct representing a watch channel.
pub struct WatchChannel {
    pub sender: WatchSender<String>,
}

impl WatchChannel {
    pub fn new() -> Self {
        let (sender, _) = watch::channel(String::new());
        Self { sender }
    }
}

/// Struct for managing watch streams.
#[derive(Clone)]
pub struct AuroraWatchStreams {
    channels: Arc<Mutex<HashMap<String, WatchChannel>>>,
    backend: Arc<dyn PubSubBackend>,
}

impl AuroraWatchStreams {
    /// Creates a new AuroraWatchStreams instance with the given backend.
    pub fn new(backend: Arc<dyn PubSubBackend>) -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
            backend,
        }
    }

    /// Creates a new watch channel.
    pub async fn create_channel(&self, channel_name: String) -> Result<(), AuroraWatchError> {
        let mut channels = self.channels.lock().await;
        if channels.contains_key(&channel_name) {
            return Ok(()); // Channel already exists
        }

        let channel = WatchChannel::new();
        channels.insert(channel_name.clone(), channel);
        drop(channels); // Release the lock early

        // Subscribe to the backend and handle incoming messages
        let backend = Arc::clone(&self.backend);
        let channels_clone = Arc::clone(&self.channels);
        tokio::spawn(async move {
            match backend.subscribe(&channel_name).await {
                Ok(mut message_stream) => {
                    while let Some(message) = message_stream.next_message().await {
                        let channels = channels_clone.lock().await;
                        if let Some(channel) = channels.get(&channel_name) {
                            let _ = channel.sender.send(message);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to subscribe to channel '{}': {}", channel_name, e);
                }
            }
        });

        Ok(())
    }

    /// Publishes a message to a watch channel.
    pub async fn publish(&self, channel_name: &str, message: &str) -> Result<(), AuroraWatchError> {
        // Removed the local send to prevent duplicate messages
        self.backend
            .publish(channel_name, message)
            .await
            .map_err(|e| AuroraWatchError::PubSubError(e.to_string()))?;
        Ok(())
    }

    /// Subscribes to a watch channel with a callback.
    pub async fn subscribe<F>(
        &self,
        channel_name: &str,
        mut callback: F,
    ) -> Result<tokio::task::JoinHandle<()>, AuroraWatchError>
    where
        F: FnMut(String) + Send + 'static,
    {
        let channels = &self.channels.lock().await;
        let channel = channels
            .get(channel_name)
            .ok_or_else(|| AuroraWatchError::ChannelNotFound(channel_name.to_string()))?;

        let mut receiver = channel.sender.subscribe();

        let handle = tokio::spawn(async move {
            loop {
                match receiver.changed().await {
                    Ok(_) => {
                        let message = receiver.borrow().clone();
                        callback(message);
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(handle)
    }
}
