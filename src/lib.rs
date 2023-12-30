use std::sync::Arc;

use aurora::AuroraStreams;

pub mod aurora;

pub fn create_stream(#[cfg(feature = "redis")] url: &str) -> &'static AuroraStreams {
    #[cfg(feature = "redis")]
    let client = redis::Client::open(url).unwrap();
    let streams = Arc::new(AuroraStreams::new(
        #[cfg(feature = "logging")]
        client,
    ));
    let a_streams: &'static AuroraStreams = Box::leak(Box::new(streams.clone()));
    a_streams
}
