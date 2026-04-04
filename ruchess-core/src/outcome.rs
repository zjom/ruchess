use crate::Color;

pub enum Outcome {
    Ongoing,
    Win(Color, WinReason),
    Draw(DrawReason),
}

pub enum WinReason {
    Checkmate,
    Resignation,
    Timeout,
}

pub enum DrawReason {
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule, // https://en.wikipedia.org/wiki/Fifty-move_rule
    Repetition,
    Agreement,
}
