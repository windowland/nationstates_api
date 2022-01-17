#[derive(thiserror::Error, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NsError<B, S> {
    #[error("Failed to build request: {0}")]
    BuildError(B),
    #[error("Failed to send request: {0}")]
    SendError(S),
}
