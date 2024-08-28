use aurora_streams::{aurora::Observer, create_stream};

const CHANNEL_NAME: &str = "my_channel";

struct MySubscriber;

impl Observer for MySubscriber {
    fn on_message(&self, message: String) {
        println!("Received message: {}", message);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = create_stream("redis://127.0.0.1").unwrap();

    stream.create_channel(CHANNEL_NAME.to_string()).await?;

    stream
        .subscribe(CHANNEL_NAME.to_string(), MySubscriber)
        .await?;

    stream
        .publish(CHANNEL_NAME.to_string(), "Hello, World!".to_string())
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(())
}
