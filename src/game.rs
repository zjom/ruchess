use crate::Role;
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

    pub fn move_(&self, from: &Square, to: &Square) -> Self {
        let board = self.board.move_(from, to);
        let turn = self.turn.opponent();

        Game { board, turn }
    }

    pub fn is_move_valid(&self, from: &Square, to: &Square) -> bool {
        match self.board.piece_at(from) {
            None => false,
            Some(Piece(_, color)) if color != self.turn => false,
            Some(Piece(piece, _)) => true,
        };

        true
    }

    // fn valid_moves(&self, piece:&PieceWithColor,square:&Square)->Bitboard{
    //     match piece.0 {
    //         Piece::Pawn => {}
    //     }
    // }

    // if !self.board.side(self.turn).contains(from) || self.board.side(self.turn).contains(to) {
    //     return false;
    // }
}
