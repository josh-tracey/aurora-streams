use aurora_streams::create_streams;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Frame {
    id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TargetSetpoint {
    frame: Frame,
    latitude: f64,
    longitude: f64,
    altitude: f32,
    yaw: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum FlightStyle {
    Approach,
    Cruise,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let streams = create_streams();

    // Create channels
    streams
        .create_channel::<TargetSetpoint>("setpoints".to_string())
        .await?;
    streams
        .create_channel::<FlightStyle>("flight_style".to_string())
        .await?;
    streams
        .create_channel::<String>("commands".to_string())
        .await?;
    streams
        .create_channel::<String>("alerts".to_string())
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Subscribe to "setpoints" channel
    let handle_setpoints = streams
        .subscribe("setpoints", move |setpoint: TargetSetpoint| {
            println!("Received Setpoint: {:?}", setpoint);
        })
        .await?;

    // Subscribe to "flight_style" channel
    let handle_flight_style = streams
        .subscribe("flight_style", move |flight_style: FlightStyle| {
            println!("Received Flight Style: {:?}", flight_style);
        })
        .await?;

    // Subscribe to "commands" channel
    let handle_commands = streams
        .subscribe("commands", move |command: String| {
            println!("Received Command: {}", command);
        })
        .await?;

    // Subscribe to "alerts" channel
    let handle_alerts = streams
        .subscribe("alerts", move |alert: String| {
            println!("Received Alert: {}", alert);
        })
        .await?;

    // Publish messages
    let setpoint = TargetSetpoint {
        frame: Frame { id: 1 },
        latitude: 37.7749,
        longitude: -122.4194,
        altitude: 500.0,
        yaw: 90.0,
    };
    streams.publish("setpoints", &setpoint).await?;

    let flight_style = FlightStyle::Cruise;
    streams.publish("flight_style", &flight_style).await?;

    let s = streams.clone();

    // Spawn a CPU-bound task using spawn_blocking
    let blocking_handle = tokio::task::spawn_blocking(move || {
        // Simulate CPU-intensive computation
        std::thread::sleep(std::time::Duration::from_secs(1));
        let command = "START_ENGINE".to_string();

        // Since we're in a blocking thread, we need a runtime to call async methods
        // Create a new current-thread runtime for this blocking task
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            s.publish("commands", &command).await.unwrap();
        });
    });

    // Wait for the blocking task to complete
    blocking_handle.await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    println!("All messages received successfully.");

    // Cleanup
    handle_setpoints.abort();
    handle_flight_style.abort();
    handle_commands.abort();
    handle_alerts.abort();

    Ok(())
}
