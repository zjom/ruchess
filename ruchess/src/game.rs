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
