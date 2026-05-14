//! # Moves
//!
//! A [`Move`] is a fully-decorated chess move: the piece, the squares
//! involved, any capture, promotion, castling, or en-passant flags, and the
//! [`Board`]s before and after the move is applied.
//!
//! Construct moves through the named constructors ([`Move::quiet`],
//! [`Move::capture`], [`Move::castle`], [`Move::enpassant`],
//! [`Move::promotion`]) rather than by hand — they fill in the
//! mutually-exclusive flag fields correctly.

use crate::{
    board::Board,
    color::Color,
    piece::Piece,
    role::{PromotableRole, Role},
    side::Side,
    square::Square,
};

/// A chess move together with all of its side effects.
///
/// Each move is either a quiet push, a capture, a castle, an en-passant
/// capture, or a promotion. The flag fields are mutually exclusive — at
/// most one of `castle`, `enpassant`, and `promotion` is `Some`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    /// The piece making the move (color is the side making the move).
    pub piece: Piece,
    /// The origin square.
    pub orig: Square,
    /// The destination square.
    pub dest: Square,
    /// The square of the captured piece, if any. For en-passant this is the
    /// captured pawn's actual square (not `dest`).
    pub capture: Option<Square>,
    /// The role this pawn promotes to, if the move is a promotion.
    pub promotion: Option<PromotableRole>,
    /// The castling side, if the move is a castle.
    pub castle: Option<Side>,
    /// `Some(())` if the move is an en-passant capture.
    pub enpassant: Option<()>,
    /// The board state resulting from applying this move.
    pub after: Board,
}

impl Move {
    /// Builds a quiet (non-capturing, non-special) move from `orig` to `dest`.
    pub fn quiet(piece: Piece, orig: Square, dest: Square, after: Board) -> Self {
        Self {
            piece,
            orig,
            dest,
            capture: None,
            promotion: None,
            castle: None,
            enpassant: None,
            after,
        }
    }

    /// Builds an ordinary capture. `captured` is the square holding the
    /// captured piece — for non-en-passant captures this equals `dest`.
    pub fn capture(
        piece: Piece,
        orig: Square,
        dest: Square,
        captured: Square,
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
        }
    }

    /// Builds a castling move on `side` for `color`.
    ///
    /// `king_from`/`king_to` describe the king's motion; the rook movement is
    /// implied by the [`Side`] and is recorded in `after`.
    pub fn castle(
        color: Color,
        side: Side,
        king_from: Square,
        king_to: Square,
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
        }
    }

    /// Builds an en-passant capture by `color`'s pawn from `orig` to `dest`,
    /// with the opposing pawn removed at `captured`.
    pub fn enpassant(
        color: Color,
        orig: Square,
        dest: Square,
        captured: Square,
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
        }
    }

    /// Builds a promotion move. `captured` is `Some(square)` when the
    /// promotion is also a capture (a diagonal pawn move into the back rank),
    /// otherwise `None` for a straight push.
    pub fn promotion(
        color: Color,
        orig: Square,
        dest: Square,
        role: PromotableRole,
        captured: Option<Square>,
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
        }
    }
}
