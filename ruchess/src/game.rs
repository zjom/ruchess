use std::fmt::Display;

use crate::{
    outcome::{DrawReason, Outcome},
    ply::Ply,
    position::Position,
    square::Square,
};

#[derive(Debug, Default)]
pub struct Game {
    position: Position,
    turn: Ply,
    outcome: Option<Outcome>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            turn: Ply::new(),
            outcome: None,
        }
    }

    pub fn mve(self, orig: Square, dest: Square) -> Option<Self> {
        self.position.mve(orig, dest).map(|position| {
            let has_moves = position.has_moves();

            let outcome = if position.history().is_threefold_repetition() {
                Some(Outcome::Draw(DrawReason::ThreeFoldRepetition))
            } else if position.history().half_moves() >= 50 {
                Some(Outcome::Draw(DrawReason::FiftyMoveRule))
            } else if position.is_check() && !has_moves {
                let winner = position.color().opponent();
                Some(Outcome::Win(winner))
            } else if !has_moves && !position.clone().has_moves() {
                Some(Outcome::Draw(DrawReason::Stalemate))
            } else {
                None
            };

            Self {
                position,
                turn: self.turn.incr(),
                outcome,
            }
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
