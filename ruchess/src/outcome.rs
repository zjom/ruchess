use crate::color::Color;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Win(Color),
    Draw(DrawReason),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DrawReason {
    Stalemate,
    ThreeFoldRepetition,
    InsufficientMaterial,
    FiftyMoveRule,
}
