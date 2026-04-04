use std::fmt::{Debug, Display};

use crate::m::{Move, pseudo_legal_moves};
use crate::outcome::Outcome;
use crate::{Board, Color, MoveError, Piece, Role, Square};

pub struct Game {
    board: Board,
    turn: Color,
    en_passant: Option<Square>,
    in_check: Option<Color>,
    outcome: Outcome,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
            en_passant: None,
            in_check: None,
            outcome: Outcome::Ongoing,
        }
    }

    pub fn validate_move(&self, mv: &Move) -> Result<(), MoveError> {
        let piece = self
            .board
            .piece_at(&mv.from)
            .ok_or(MoveError::NoPiece(mv.from))?;

        let Piece(_, color) = piece;
        if color != self.turn {
            return Err(MoveError::WrongColor(mv.from, color));
        }

        let candidates = pseudo_legal_moves(piece, &mv.from, &self.board, self.en_passant);
        if !candidates.contains(&mv.to) {
            return Err(MoveError::IllegalMove(mv.to));
        }

        Ok(())
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn grid(&self) -> [[Option<Piece>; 8]; 8] {
        self.board.as_grid()
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), MoveError> {
        self.validate_move(&mv)?;
        let prev_en_passant = self.en_passant;
        if let Some(piece) = self.board.remove_at(&mv.from) {
            self.en_passant = match piece {
                Piece(Role::Pawn, Color::White) if mv.to.0 == mv.from.0 + 16 => {
                    Some(Square(mv.from.0 + 8)) // the square white passed through
                }
                Piece(Role::Pawn, Color::Black) if mv.from.0 == mv.to.0 + 16 => {
                    Some(Square(mv.from.0 - 8)) // the square black passed through
                }
                _ => None,
            };
            self.board.set_at(&mv.to, piece);

            if matches!(piece, Piece(Role::Pawn, _)) && Some(mv.to) == prev_en_passant {
                // the captured pawn sits one rank behind the destination (from mover's perspective)
                let captured_pawn_sq = match self.turn {
                    // turn is still the mover's color here
                    Color::White => Square(mv.to.0 - 8),
                    Color::Black => Square(mv.to.0 + 8),
                };
                self.board.discard_at(&captured_pawn_sq);
            }
        }
        self.turn = self.turn.opponent();
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "{} to move.", self.turn)
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "en_passant: {:?}", self.en_passant)?;
        writeln!(f, "{} to move.", self.turn)
    }
}
