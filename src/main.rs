use aurora_streams::create_stream;

fn test_observer(message: String) {
    println!("1 Message recieved: {}", message);
}

#[tokio::main]
async fn main() {
    let stream = create_stream("redis://localhost:6379");

    let test_channel = stream.create_channel("test_channel".to_string()).await;

    let task_1 = stream
        .subscribe("test_channel".to_string(), test_observer)
        .await;

    let task_2 = stream
        .subscribe("test_channel".to_string(), |message| {
            println!("2 Message recieved: {}", message);
        })
        .await;

    stream
        .publish("test_channel".to_string(), "Hello World!".to_string())
        .await;

    for handle in vec![test_channel, task_1, task_2] {
        handle.await.unwrap();
    }
}
