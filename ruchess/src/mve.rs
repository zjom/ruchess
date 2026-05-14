use crate::{
    board::Board,
    color::Color,
    piece::Piece,
    role::{PromotableRole, Role},
    side::Side,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub orig: Square,
    pub dest: Square,
    pub capture: Option<Square>,
    pub promotion: Option<PromotableRole>,
    pub castle: Option<Side>,
    pub enpassant: Option<()>,
    pub after: Board,
    pub previous: Option<Board>,
}

impl Move {
    pub fn quiet(piece: Piece, orig: Square, dest: Square, prev: Board, after: Board) -> Self {
        Self {
            piece,
            orig,
            dest,
            capture: None,
            promotion: None,
            castle: None,
            enpassant: None,
            after,
            previous: Some(prev),
        }
    }

    pub fn capture(
        piece: Piece,
        orig: Square,
        dest: Square,
        captured: Square,
        prev: Board,
        after: Board,
    ) -> Self {
        Self {
            piece,
            orig,
            dest,
            capture: Some(captured),
            promotion: None,
            castle: None,
            enpassant: None,
            after,
            previous: Some(prev),
        }
    }

    pub fn castle(
        color: Color,
        side: Side,
        king_from: Square,
        king_to: Square,
        prev: Board,
        after: Board,
    ) -> Self {
        Self {
            piece: Piece {
                role: Role::King,
                color,
            },
            orig: king_from,
            dest: king_to,
            capture: None,
            promotion: None,
            castle: Some(side),
            enpassant: None,
            after,
            previous: Some(prev),
        }
    }

    pub fn enpassant(
        color: Color,
        orig: Square,
        dest: Square,
        captured: Square,
        prev: Board,
        after: Board,
    ) -> Self {
        Self {
            piece: Piece {
                role: Role::Pawn,
                color,
            },
            orig,
            dest,
            capture: Some(captured),
            promotion: None,
            castle: None,
            enpassant: Some(()),
            after,
            previous: Some(prev),
        }
    }

    pub fn promotion(
        color: Color,
        orig: Square,
        dest: Square,
        role: PromotableRole,
        captured: Option<Square>,
        prev: Board,
        after: Board,
    ) -> Self {
        Self {
            piece: Piece {
                role: Role::Pawn,
                color,
            },
            orig,
            dest,
            capture: captured,
            promotion: Some(role),
            castle: None,
            enpassant: None,
            after,
            previous: Some(prev),
        }
    }
}
