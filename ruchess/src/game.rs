use std::fmt::Display;

use crate::{
    bitboard::Bitboard,
    board::Board,
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

            let outcome = if is_insufficient_material(position.board()) {
                Some(Outcome::Draw(DrawReason::InsufficientMaterial))
            } else if position.history().is_threefold_repetition() {
                Some(Outcome::Draw(DrawReason::ThreeFoldRepetition))
            } else if position.history().half_moves() >= 50 {
                Some(Outcome::Draw(DrawReason::FiftyMoveRule))
            } else if !has_moves && position.is_check() {
                let winner = position.color().opponent();
                Some(Outcome::Win(winner))
            } else if !has_moves {
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

    pub fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn ply(&self) -> Ply {
        self.turn
    }
}
fn is_insufficient_material(board: &Board) -> bool {
    if board.pawns().is_non_empty() || board.rooks().is_non_empty() || board.queens().is_non_empty()
    {
        return false;
    }

    let minors = (board.knights() | board.bishops()).count();
    if minors <= 1 {
        return true;
    }
    if board.bishops().is_empty() && board.knights().count() == 2 {
        return true;
    }

    if board.knights().is_empty() {
        let bishops = board.bishops();
        return (bishops & Bitboard::LIGHT).is_empty() || (bishops & Bitboard::DARK).is_empty();
    }

    false
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position.board())?;
        match self.outcome {
            Some(outcome) => writeln!(f, "{}", outcome),
            None => writeln!(
                f,
                "{:?} to move (move {})",
                self.turn.turn(),
                self.turn.full_move_number()
            ),
        }
    }
}
