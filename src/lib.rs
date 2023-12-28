use std::sync::Arc;

use aurora::AuroraStreams;

pub mod aurora;

pub fn create_stream(url: &str) -> &'static AuroraStreams {
    let client = redis::Client::open(url).unwrap();
    let streams = Arc::new(AuroraStreams::new(client));
    let a_streams: &'static AuroraStreams = Box::leak(Box::new(streams.clone()));
    a_streams
}

