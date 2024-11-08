use aurora_streams::{
    AuroraBroadcastError, AuroraBroadcastStreams, InMemoryBackend, PubSubBackend,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), AuroraBroadcastError> {
    let backend: Arc<dyn PubSubBackend> = Arc::new(InMemoryBackend::new());

    // Initialize AuroraBroadcastStreams
    let aurora_broadcast = AuroraBroadcastStreams::new(backend);

    // Create a broadcast channel
    let channel_name = "broadcast_channel".to_string();
    aurora_broadcast
        .create_channel(channel_name.clone())
        .await?;

    // Subscribe to the channel
    let handle = aurora_broadcast
        .subscribe(&channel_name, |msg| {
            println!("Broadcast Received message: {}", msg);
        })
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // Publish a message
    aurora_broadcast
        .publish(&channel_name, "Hello, Broadcast Streams!")
        .await?;

    let a = aurora_broadcast.clone();

    // Wait to receive the message
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    a.publish(&channel_name, "Hello, Broadcast Streams! 2")
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // Optionally, abort the subscription
    handle.abort();

    Ok(())
}
