pub mod aurora_streams;
pub mod backend;
pub mod errors;

use std::sync::Arc;

pub use aurora_streams::AuroraStreams;
pub use backend::{InMemoryBackend, MessageStream, PubSubBackend};
pub use errors::{AuroraBroadcastError, AuroraWatchError};

pub fn create_streams() -> Arc<AuroraStreams> {
    let backend = Arc::new(InMemoryBackend::new());
    Arc::new(AuroraStreams::new(backend))
}
