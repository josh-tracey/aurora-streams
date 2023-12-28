use futures_util::StreamExt;
use redis::{AsyncCommands, Client, Value};
use std::collections::HashMap;
use tokio::sync::watch::{self, channel};
use tokio::sync::Mutex;

pub struct Channel {
    pub sender: watch::Sender<String>,
    pub receiver: watch::Receiver<String>,
}

impl Channel {
    pub fn new() -> Self {
        let (sender, receiver) = channel(String::new());
        Self { sender, receiver }
    }
}

pub struct AuroraStreams {
    channels: Mutex<HashMap<String, Channel>>,
    redis_client: Client,
}

impl AuroraStreams {
    pub fn new(redis_client: Client) -> Self {
        Self {
            channels: Mutex::new(HashMap::<String, Channel>::new()),
            redis_client,
        }
    }

    pub async fn create_channel(
        &'static self,
        channel_name: String,
    ) -> tokio::task::JoinHandle<()> {
        let channel = Channel::new();
        let mut channels = self.channels.lock().await;
        channels.insert(channel_name.clone(), channel);

        let mut pub_sub = self
            .redis_client
            .get_async_connection()
            .await
            .unwrap()
            .into_pubsub();
        pub_sub.subscribe(channel_name.clone()).await.unwrap();

        let handle = tokio::spawn(async move {
            while let Some(msg) = pub_sub.on_message().next().await {
                let channel_name = msg.get_channel_name().to_string();
                let message: String = msg.get_payload().unwrap();
                let channels = self.channels.lock().await;
                let sender = &channels.get(&channel_name).unwrap().sender;
                sender.send(message).unwrap();
            }
        });

        return handle;
    }

    pub async fn publish(&self, channel_name: String, message: String) {
        let channels = self.channels.lock().await;
        let sender = &channels.get(&channel_name).unwrap().sender;
        sender.send(message.clone()).unwrap();
        let mut con = self.redis_client.get_async_connection().await.unwrap();
        con.publish::<&str, &str, Value>(&channel_name, &message)
            .await
            .unwrap();
    }

    pub async fn subscribe<'a>(
        &'a self,
        channel_name: String,
        observer: impl Fn(String) + Send + 'a + 'static,
    ) -> tokio::task::JoinHandle<()> {
        let channels = self.channels.lock().await;
        let mut receiver = channels.get(&channel_name).unwrap().receiver.clone();

        let handle = tokio::spawn(async move {
            let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
            tokio::pin!(sleep);
            let mut previous_message: Option<String> = None;
            loop {
                tokio::select! {
                    _ = receiver.changed() => {
                        let message = receiver.borrow_and_update();
                        if message.to_string() != previous_message.clone().unwrap_or_default() {
                            observer(message.to_string());
                        }
                    previous_message = Some(message.to_string());
                    }
                    _ = &mut sleep => {}
                }
            }
        });

        return handle;
    }
}
