//! # Game Outcomes
//!
//! Terminal results of a chess game: a win for one side or a draw with a
//! specific reason. [`Game`](crate::game::Game) attaches an [`Outcome`] to
//! its state as soon as a final position is reached.

use std::fmt::Display;

use crate::color::Color;

/// The terminal result of a chess game.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    /// One side checkmated the other. The wrapped [`Color`] is the winner.
    Win(Color),
    /// The game ended in a draw, with the specific [`DrawReason`].
    Draw(DrawReason),
}

/// Why a drawn game ended in a draw.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawReason {
    /// The side to move has no legal moves and is not in check.
    Stalemate,
    /// The current position has occurred at least three times.
    ThreeFoldRepetition,
    /// Neither side has sufficient material to deliver checkmate.
    InsufficientMaterial,
    /// 50 reversible half-moves have passed without a pawn move or capture.
    FiftyMoveRule,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win(color) => writeln!(f, "{} wins!", color),
            Self::Draw(reason) => writeln!(f, "draw by {}", reason),
        }
    }
}

impl Display for DrawReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stalemate => writeln!(f, "stalemate"),
            Self::ThreeFoldRepetition => writeln!(f, "three fold repetition"),
            Self::InsufficientMaterial => writeln!(f, "insufficient material"),
            Self::FiftyMoveRule => writeln!(f, "fifty move rule"),
        }
    }
}
