## Aurora Streams

[![Rust](https://github.com/josh-tracey/aurora-streams/actions/workflows/rust.yml/badge.svg)](https://github.com/josh-tracey/aurora-streams/actions/workflows/rust.yml)

A Rust library for managing publish-subscribe channels using Tokio messages passing channels.

optionally event routing can be enabled using redis by passing the `event-routing` feature flag.

this allows pub-sub messages to be shared between multiple services.

### Features:

* Simplified Pub/Sub Interaction
* Asynchronous Operations
* Duplicate Message Handling
* Channel Management

## Getting Started:

Add the dependency to your Cargo.toml:

```toml
[dependencies]
aurora-streams = "0.1.0"  # Replace with the actual version
```

Import the library in your Rust code:

```rust
use aurora_streams::create_stream;
```

### Usage:


#### Create an AuroraStreams instance:

```rust
let streams = create_stream()
```

#### Create a channel:

```rust
streams.create_channel("test_channel".to_string()).await;
```

#### Publish a message:

```rust
streams.publish("test_channel".to_string(), "Hello World!".to_string()).await;
```

#### Subscribe to a channel:

```rust
streams.subscribe("test_channel".to_string(), |message| {
    println!("Message received: {}", message);
}).await;
```

### License:

This library is licensed under the MIT License.

### Contributing:

Contributions are welcome! Please see the contributing guidelines for details.
