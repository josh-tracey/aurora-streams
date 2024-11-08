// src/lib.rs

pub mod backend;
pub mod broadcast_streams;
pub mod errors;
pub mod watch_streams;

pub use backend::{InMemoryBackend, MessageStream, PubSubBackend};
pub use broadcast_streams::AuroraBroadcastStreams;
pub use errors::{AuroraBroadcastError, AuroraWatchError};
pub use watch_streams::AuroraWatchStreams;
