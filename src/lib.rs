use std::sync::Arc;

use aurora::AuroraStreams;

pub mod aurora;

pub fn create_stream(#[cfg(feature = "event-routing")] url: &str) -> &'static AuroraStreams {
    #[cfg(feature = "event-routing")]
    let client = redis::Client::open(url).unwrap();
    let streams = Arc::new(AuroraStreams::new(
        #[cfg(feature = "event-routing")]
        client,
    ));
    let a_streams: &'static AuroraStreams = Box::leak(Box::new(streams.clone()));
    a_streams
}
