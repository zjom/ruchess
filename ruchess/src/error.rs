use crate::{Color, Square};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid square notation: `{0}`")]
pub struct ParseSquareError(pub String);

#[derive(Debug, Error)]
pub enum MoveError {
    #[error("no piece at {0}")]
    NoPiece(Square),
    #[error("piece at {0} belongs to {1}")]
    WrongColor(Square, Color),
    #[error("piece cannot move to {0}")]
    IllegalMove(Square),
}
