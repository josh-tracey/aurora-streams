use crate::backend::PubSubBackend;
use crate::errors::AuroraBroadcastError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender as BroadcastSender};
use tokio::sync::Mutex;

/// Struct representing a broadcast channel.
#[derive(Clone)]
pub struct BroadcastChannel {
    pub sender: BroadcastSender<String>,
}

impl BroadcastChannel {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100); // Buffer size can be adjusted
        Self { sender }
    }
}

/// Struct for managing broadcast streams.
#[derive(Clone)]
pub struct AuroraBroadcastStreams {
    channels: Arc<Mutex<HashMap<String, BroadcastChannel>>>,
    backend: Arc<dyn PubSubBackend>,
}

impl AuroraBroadcastStreams {
    /// Creates a new AuroraBroadcastStreams instance with the given backend.
    pub fn new(backend: Arc<dyn PubSubBackend>) -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
            backend,
        }
    }

    /// Creates a new broadcast channel.
    pub async fn create_channel(&self, channel_name: String) -> Result<(), AuroraBroadcastError> {
        let mut channels = self.channels.lock().await;
        if channels.contains_key(&channel_name) {
            return Ok(()); // Channel already exists
        }

        let channel = BroadcastChannel::new();
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

    /// Publishes a message to a broadcast channel.
    pub async fn publish(
        &self,
        channel_name: &str,
        message: &str,
    ) -> Result<(), AuroraBroadcastError> {
        self.backend
            .publish(channel_name, message)
            .await
            .map_err(|e| AuroraBroadcastError::PubSubError(e.to_string()))?;
        Ok(())
    }

    /// Subscribes to a broadcast channel with a callback.
    pub async fn subscribe<F>(
        &self,
        channel_name: &str,
        mut callback: F,
    ) -> Result<tokio::task::JoinHandle<()>, AuroraBroadcastError>
    where
        F: FnMut(String) + Send + 'static,
    {
        let channels = self.channels.lock().await;
        let channel = channels
            .get(channel_name)
            .ok_or_else(|| AuroraBroadcastError::ChannelNotFound(channel_name.to_string()))?
            .clone();
        drop(channels); // Release the lock early

        let mut receiver = channel.sender.subscribe();

        let handle = tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(message) => callback(message),
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue, // Handle lagging
                }
            }
        });

        Ok(handle)
    }
}
