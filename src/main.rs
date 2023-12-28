use std::sync::Arc;

use aurora_streams::AuroraStreams;

fn test_observer(message: String) {
    println!("1 Message recieved: {}", message);
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://localhost:6379").unwrap();

    let streams = Arc::new(AuroraStreams::new(client));
    let a_streams: &'static AuroraStreams = Box::leak(Box::new(streams.clone()));

    let test_channel = a_streams.create_channel("test_channel".to_string()).await;

    a_streams
        .publish("test_channel".to_string(), "Hello World!".to_string())
        .await;

    let task_1 = a_streams
        .subscribe("test_channel".to_string(), test_observer)
        .await;

    let task_2 = a_streams
        .subscribe("test_channel".to_string(), |message| {
            println!("2 Message recieved: {}", message);
        })
        .await;

    for handle in vec![test_channel, task_1, task_2] {
        handle.await.unwrap();
    }
}
