use thiserror::Error;

/// Error type for AuroraBroadcastStreams.
#[derive(Error, Debug)]
pub enum AuroraBroadcastError {
    #[error("PubSub error: {0}")]
    PubSubError(String),

    #[error("Channel '{0}' does not exist")]
    ChannelNotFound(String),

    #[error("Failed to send message: {0}")]
    SendError(String),
}

/// Error type for AuroraWatchStreams.
#[derive(Error, Debug)]
pub enum AuroraWatchError {
    #[error("PubSub error: {0}")]
    PubSubError(String),

    #[error("Channel '{0}' does not exist")]
    ChannelNotFound(String),

    #[error("Failed to send message: {0}")]
    SendError(String),
}
