use std::fmt::Display;

use crate::{Board, Color, Piece, Square};

pub struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
        }
    }

    pub fn move_(mut self, from: &Square, to: &Square) -> Self {
        if let Some(piece) = self.board.remove_at(&from) {
            self.board.set_at(&to, piece);
        }

        self.turn = self.turn.opponent();
        self
    }

    pub fn is_move_valid(&self, from: &Square, to: &Square) -> bool {
        match self.board.piece_at(from) {
            None => false,
            Some(Piece(_, color)) if color != self.turn => false,
            Some(Piece(_, _)) => true,
        };

        true
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "{} to move.", self.turn)
    }
}
