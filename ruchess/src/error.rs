use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid square notation: `{0}`")]
pub struct ParseSquareError(pub String);
