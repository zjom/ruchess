use std::fmt::Display;

use crate::{ply::Ply, position::Position, square::Square};

#[derive(Debug, Default)]
pub struct Game {
    position: Position,
    turn: Ply,
}

impl Game {
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            turn: Ply::new(),
        }
    }

    pub fn mve(self, orig: Square, dest: Square) -> Option<Self> {
        self.position.mve(orig, dest).map(|position| Self {
            position,
            turn: self.turn.incr(),
        })
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position.board())?;
        writeln!(
            f,
            "{:?} to move (move {})",
            self.turn.turn(),
            self.turn.full_move_number()
        )
    }
}
