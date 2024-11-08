use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuroraBroadcastError {
    #[error("Channel not found: {0}")]
    ChannelNotFound(String),

    #[error("Type mismatch for channel: {0}")]
    TypeMismatch(String),

    #[error("PubSub error: {0}")]
    PubSubError(String),
}

#[derive(Error, Debug)]
pub enum AuroraWatchError {
    #[error("Channel not found: {0}")]
    ChannelNotFound(String),

    #[error("Type mismatch for channel: {0}")]
    TypeMismatch(String),

    #[error("PubSub error: {0}")]
    PubSubError(String),
}
