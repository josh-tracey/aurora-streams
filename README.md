# Aurora Streams

[![Rust](https://github.com/josh-tracey/aurora-streams/actions/workflows/rust.yml/badge.svg)](https://github.com/josh-tracey/aurora-streams/actions/workflows/rust.yml)

**Aurora Streams** is a **Rust library** designed to simplify the management of **publish-subscribe (Pub/Sub)** channels using **Tokio's** asynchronous message-passing capabilities. It offers **type-safe**, **asynchronous**, and **flexible** communication channels, making it easier to build robust and scalable applications.

---
## 📋 Table of Contents

1. [Features](#features)
2. [Getting Started](#getting-started)
   - [Installation](#installation)
   - [Importing the Library](#importing-the-library)
3. [Usage](#usage)
   - [Creating an AuroraStreams Instance](#creating-an-aurorastreams-instance)
     - [Using In-Memory Backend](#using-in-memory-backend)
   - [Defining Message Types](#defining-message-types)
   - [Creating Channels](#creating-channels)
   - [Publishing Messages](#publishing-messages)
   - [Subscribing to Channels](#subscribing-to-channels)
4. [Advanced Usage](#advanced-usage)
   - [Handling Multiple Message Types](#handling-multiple-message-types)
5. [License](#license)
6. [Contributing](#contributing)
7. [Contact](#contact)

---
## 🎉 Features

- **Type-Safe Pub/Sub Interaction**: Define channels with specific message types, ensuring compile-time type safety.
- **Asynchronous Operations**: Utilize Tokio for efficient asynchronous task handling and message processing.
- **Channel Management**: Easily create, publish to, and subscribe from multiple channels.
- **Flexible Backend Support**: Choose between in-memory backend for local communication.
- **Serialization Support**: Leverage Serde for seamless serialization and deserialization of messages.

---
## 🚀 Getting Started

### 🛠 Installation

Add Aurora Streams to your `Cargo.toml`:

```toml
[dependencies]
aurora-streams = "0.1.0"  # Replace with the actual version
```

---

### Importing the Library

In your Rust code, import Aurora Streams and necessary components:
```rust
use aurora_streams::create_streams;
```

---

### Usage
#### Creating an AuroraStreams Instance
Using In-Memory Backend

For local, multi-thread communication, you can use the in-memory backend, which is the default option. To create an AuroraStreams instance with the in-memory backend, call the `create_streams` function:

```rust
use aurora_streams::create_streams;

let streams = create_streams();
```

---

### Defining Message Types

Define the data structures you wish to publish and subscribe to using Serde for serialization and deserialization:

```rust

use serde::{Serialize, Deserialize};

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
```

---

### Creating Channels

Create channels for different message types. Each channel is associated with a specific data type, ensuring type safety.

```rust
// Create channels for different message types
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
```

---

### Publishing Messages

Publish messages to specific channels. Ensure that the message type matches the channel's defined type.

```rust
// Publish a TargetSetpoint message
let setpoint = TargetSetpoint {
    frame: Frame { id: 1 },
    latitude: 37.7749,
    longitude: -122.4194,
    altitude: 500.0,
    yaw: 90.0,
};
streams.publish("setpoints", &setpoint).await?;

// Publish a FlightStyle message
let flight_style = FlightStyle::Cruise;
streams.publish("flight_style", &flight_style).await?;

// Publish a command
let command = "START_ENGINE".to_string();
streams.publish("commands", &command).await?;

// Publish an alert
let alert = "Battery low".to_string();
streams.publish("alerts", &alert).await?;
```

---

### Subscribing to Channels

Subscribe to specific channels to receive and handle incoming messages. The callback receives messages of the channel's defined type.

```rust
use tokio::sync::oneshot;

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
```

    Note: Each subscribe call returns a JoinHandle which can be used to manage the subscription task, such as aborting it when no longer needed.

---

### License

This library is licensed under the MIT License. See the LICENSE file for details.

---

### Additional Resources

- Tokio Documentation: https://docs.rs/tokio
- Rust Async Programming: https://rust-lang.github.io/async-book/
- Serde Documentation: https://serde.rs/
