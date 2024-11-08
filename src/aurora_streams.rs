use crate::backend::PubSubBackend;
use crate::errors::AuroraBroadcastError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct AuroraStreams {
    backend: Arc<dyn PubSubBackend>,
    channels: Arc<Mutex<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl AuroraStreams {
    pub fn new(backend: Arc<dyn PubSubBackend>) -> Self {
        Self {
            backend,
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_channel<T>(&self, channel_name: String) -> Result<(), AuroraBroadcastError>
    where
        T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    {
        let mut channels = self.channels.lock().await;
        if channels.contains_key(&channel_name) {
            return Err(AuroraBroadcastError::PubSubError(format!(
                "Channel '{}' already exists",
                channel_name
            )));
        }

        let typed_channel = TypedChannel::<T>::new(Arc::clone(&self.backend), channel_name.clone());
        channels.insert(channel_name, Box::new(typed_channel));
        Ok(())
    }

    pub async fn publish<T>(
        &self,
        channel_name: &str,
        message: &T,
    ) -> Result<(), AuroraBroadcastError>
    where
        T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    {
        let channels = self.channels.lock().await;
        let channel = channels
            .get(channel_name)
            .ok_or_else(|| AuroraBroadcastError::ChannelNotFound(channel_name.to_string()))?;
        if let Some(typed_channel) = channel.downcast_ref::<TypedChannel<T>>() {
            typed_channel
                .publish(message)
                .await
                .map_err(|e| AuroraBroadcastError::PubSubError(e.to_string()))
        } else {
            Err(AuroraBroadcastError::TypeMismatch(channel_name.to_string()))
        }
    }

    pub async fn subscribe<T, F>(
        &self,
        channel_name: &str,
        callback: F,
    ) -> Result<JoinHandle<()>, AuroraBroadcastError>
    where
        T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
        F: FnMut(T) + Send + 'static,
    {
        let channels = self.channels.lock().await;
        let channel = channels
            .get(channel_name)
            .ok_or_else(|| AuroraBroadcastError::ChannelNotFound(channel_name.to_string()))?;
        if let Some(typed_channel) = channel.downcast_ref::<TypedChannel<T>>() {
            typed_channel
                .subscribe(callback)
                .await
                .map_err(|e| AuroraBroadcastError::PubSubError(e.to_string()))
        } else {
            Err(AuroraBroadcastError::TypeMismatch(channel_name.to_string()))
        }
    }
}

pub struct TypedChannel<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    backend: Arc<dyn PubSubBackend>,
    channel_name: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T> TypedChannel<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    pub fn new(backend: Arc<dyn PubSubBackend>, channel_name: String) -> Self {
        Self {
            backend,
            channel_name,
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn publish(&self, message: &T) -> Result<(), Box<dyn Error + Send + Sync>> {
        let serialized = serde_json::to_string(message)?;
        self.backend.publish(&self.channel_name, &serialized).await
    }

    pub async fn subscribe<F>(
        &self,
        mut callback: F,
    ) -> Result<JoinHandle<()>, Box<dyn Error + Send + Sync>>
    where
        F: FnMut(T) + Send + 'static,
    {
        let mut message_stream = self.backend.subscribe(&self.channel_name).await?;
        let channel_name = self.channel_name.clone();

        let handle = tokio::spawn(async move {
            while let Some(serialized_msg) = message_stream.next_message().await {
                match serde_json::from_str::<T>(&serialized_msg) {
                    Ok(msg) => callback(msg),
                    Err(err) => eprintln!(
                        "Failed to deserialize message on channel '{}': {}",
                        channel_name, err
                    ),
                }
            }
        });

        Ok(handle)
    }
}
