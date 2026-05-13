use crate::color::Color;

/// a half-move, that is a move of one side only.
/// When we speak of a 6 ply search, we mean three full moves.
#[derive(Debug, Default)]
pub struct Ply(u32);

impl Ply {
    pub fn new() -> Self {
        Self(0)
    }

    /// Whose turn is it now
    pub fn turn(&self) -> Color {
        if self.is_even() {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn incr(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn full_move_number(&self) -> u32 {
        1 + self.0 / 2
    }

    fn is_even(&self) -> bool {
        (self.0 & 1) == 0
    }
}
