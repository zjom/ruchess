//! # Unmoved Rooks
//!
//! Tracks which rooks have not yet moved from their original starting squares.
//! This information is critical for determining castling rights.
//!
//! ### Castling Eligibility
//! In chess, a player can only castle if:
//! 1. The king has never moved.
//! 2. The chosen rook has never moved.
//!
//! While the king's movement is often tracked separately (or by checking if any
//! unmoved rooks remain for that color), [`UnmovedRooks`] specifically maintains
//! a bitmask of squares that currently contain a rook that has never left its
//! initial position.
//!
//! ---
//!
//! ## How it Works
//!
//! [`UnmovedRooks`] is a specialized [`Bitboard`] wrapper. It starts with bits
//! set at the four corners of the board (A1, H1, A8, and H8). Whenever a move
//! occurs, the bits corresponding to both the **origin** and **destination**
//! squares are cleared.
//!
//! * **Origin:** If a rook moves, it is no longer "unmoved".
//! * **Destination:** If an unmoved rook is captured, it is removed from the board,
//!   and thus can no longer be used for castling.
//!
//! ## Example
//!
//! ```
//! # use ruchess::unmoved_rooks::UnmovedRooks;
//! # use ruchess::square;
//! # use ruchess::mve::Move;
//! # use ruchess::board::Board;
//! # use ruchess::piece::Piece;
//! # use ruchess::role::Role;
//! # use ruchess::color::Color;
//! let mut ur = UnmovedRooks::standard();
//!
//! // Initially, all four rooks are "unmoved"
//! assert!(ur.contains(square::A1));
//! assert!(ur.contains(square::H1));
//! assert!(ur.contains(square::A8));
//! assert!(ur.contains(square::H8));
//!
//! // Suppose white moves the rook from A1 to A3
//! # let board = Board::new();
//! # let m = Move::quiet(
//! #     Piece { role: Role::Rook, color: Color::White },
//! #     square::A1,
//! #     square::A3,
//! #     board,
//! #     board
//! # );
//! ur = ur.update(&m);
//!
//! assert!(!ur.contains(square::A1)); // No longer unmoved
//! assert!(ur.contains(square::H1));  // H1 rook is still unmoved
//! ```

use crate::{bitboard::Bitboard, board::Board, color::Color, mve::Move, square::Square};

/// A bitmask of squares that contain rooks that have never moved.
///
/// This structure is used to track castling rights by maintaining a [`Bitboard`]
/// of original rook squares that have not been the origin or destination of any move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnmovedRooks(Bitboard);

impl UnmovedRooks {
    /// All four rooks unmoved — the standard starting position.
    /// Bits set: A1, H1, A8, H8.
    const INIT: UnmovedRooks = UnmovedRooks(Bitboard(0x8100000000000081));

    const NONE: UnmovedRooks = UnmovedRooks(Bitboard::EMPTY);

    /// Returns an [`UnmovedRooks`] instance representing the standard starting position.
    ///
    /// The squares A1, H1, A8, and H8 are initially marked as containing unmoved rooks.
    ///
    /// # Example
    /// ```
    /// # use ruchess::unmoved_rooks::UnmovedRooks;
    /// # use ruchess::square;
    /// let ur = UnmovedRooks::standard();
    /// assert!(ur.contains(square::A1));
    /// assert!(ur.contains(square::H8));
    /// ```
    pub fn standard() -> Self {
        Self::INIT
    }

    /// Scans the given [`Board`] to identify rooks that are currently on their starting ranks.
    ///
    /// This is useful when initializing a game from a specific FEN string or board state
    /// where castling rights need to be inferred or synchronized.
    ///
    /// # Example
    /// ```
    /// # use ruchess::unmoved_rooks::UnmovedRooks;
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// let ur = UnmovedRooks::from_board(board);
    /// assert!(ur.contains(square::A1));
    /// assert!(ur.contains(square::H8));
    /// ```
    pub fn from_board(board: Board) -> Self {
        let wr = board.rooks() & board.white() & Color::White.back_rank();
        let br = board.rooks() & board.black() & Color::Black.back_rank();
        Self(wr | br)
    }

    /// Updates the unmoved rooks state based on a [`Move`].
    ///
    /// This method will clear the bits for both the origin and destination squares
    /// of the move. If a rook moves from its starting square, or if a piece is
    /// captured on a starting rook square, that square is no longer considered
    /// to have an unmoved rook.
    ///
    /// # Example
    /// ```
    /// # use ruchess::unmoved_rooks::UnmovedRooks;
    /// # use ruchess::mve::Move;
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let ur = UnmovedRooks::standard();
    /// # let board = Board::new();
    /// # let m = Move::quiet(
    /// #     Piece { role: Role::Rook, color: Color::White },
    /// #     square::A1,
    /// #     square::A3,
    /// #     board,
    /// #     board
    /// # );
    /// let updated = ur.update(&m);
    /// assert!(!updated.contains(square::A1));
    /// assert!(updated.contains(square::H1));
    /// ```
    pub fn update(self, m: &Move) -> Self {
        UnmovedRooks(!(Bitboard::from(m.orig) | Bitboard::from(m.dest)) & self)
    }

    /// Returns `true` if there are no more unmoved rooks remaining.
    ///
    /// # Example
    /// ```
    /// # use ruchess::unmoved_rooks::UnmovedRooks;
    /// let ur = UnmovedRooks::standard();
    /// assert!(!ur.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        *self == Self::NONE
    }

    /// Returns `true` if the specified [`Square`] contains an unmoved rook.
    ///
    /// # Example
    /// ```
    /// # use ruchess::unmoved_rooks::UnmovedRooks;
    /// # use ruchess::square;
    /// let ur = UnmovedRooks::standard();
    /// assert!(ur.contains(square::A1));
    /// assert!(!ur.contains(square::E2));
    /// ```
    pub fn contains(&self, sq: Square) -> bool {
        self.0.is_set(sq)
    }
}

impl From<UnmovedRooks> for Bitboard {
    /// Converts the [`UnmovedRooks`] state into a raw [`Bitboard`].
    fn from(value: UnmovedRooks) -> Self {
        value.0
    }
}
