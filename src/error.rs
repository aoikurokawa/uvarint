use thiserror::Error;

#[derive(Debug, Error)]
pub enum UVarintError {
    #[error("Incomplete varint data")]
    Incomplete,

    #[error("Varint overflow - value too large")]
    Overflow,

    #[error("Buffer too small")]
    BufferTooSmall,

    #[error("Invalid UTF-8 string")]
    InvalidUtf8,

    #[error("Write operation failed")]
    WriteFailed,
}
