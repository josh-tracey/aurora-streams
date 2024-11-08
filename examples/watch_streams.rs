use aurora_streams::{AuroraWatchError, AuroraWatchStreams, InMemoryBackend, PubSubBackend};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), AuroraWatchError> {
    #[cfg(not(feature = "event-routing"))]
    let backend: Arc<dyn PubSubBackend> = Arc::new(InMemoryBackend::new());

    // Initialize AuroraWatchStreams
    let aurora_watch = AuroraWatchStreams::new(backend);

    // Create a watch channel
    let channel_name = "watch_channel".to_string();
    aurora_watch.create_channel(channel_name.clone()).await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // Subscribe to the channel
    let handle = aurora_watch
        .subscribe(&channel_name, |msg| {
            println!("Watch Received message: {}", msg);
        })
        .await?;

    // Publish a message
    aurora_watch
        .publish(&channel_name, "Hello, Watch Streams!")
        .await?;

    let a = aurora_watch.clone();
    // Wait to receive the message
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    a.publish(&channel_name, "Hello, Watch Streams! 2").await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Optionally, abort the subscription
    handle.abort();

    Ok(())
}
