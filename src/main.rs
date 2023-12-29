use std::error::Error;

use aurora_streams::create_stream;

fn test_observer(message: String) {
    println!("1 Message recieved: {}", message);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = create_stream("redis://localhost:6379");

    let _ = stream.create_channel("test_channel".to_string()).await;
    let _ = stream.create_channel("test_channel2".to_string()).await;

    let task_1 = stream
        .subscribe("test_channel".to_string(), test_observer)
        .await;

    let task_2 = stream
        .subscribe("test_channel2".to_string(), |message| {
            println!("3 Message recieved: {}", message);
        })
        .await;

    let task_3 = stream
        .subscribe("test_channel".to_string(), |message| {
            println!("2 Message recieved: {}", message);
        })
        .await;

    stream
        .publish("test_channel".to_string(), "Hello World!".to_string())
        .await?;

    stream
        .publish("test_channel2".to_string(), "What's up".to_string())
        .await?;

    for handle in vec![task_1, task_2, task_3] {
        handle?.await?;
    }

    Ok(())
}
