use std::fmt::Display;

use crate::color::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Win(Color),
    Draw(DrawReason),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawReason {
    Stalemate,
    ThreeFoldRepetition,
    InsufficientMaterial,
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
