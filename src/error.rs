use thiserror::Error;

#[derive(Debug, Error)]
pub enum UVarintError {
    #[error("Incomplete")]
    Incomplete,

    #[error("Overflow")]
    Overflow,

    #[error("BufferTooSmall")]
    BufferTooSmall,
}
