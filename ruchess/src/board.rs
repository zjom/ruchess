//! # Board
//!
//! The [`Board`] struct is the central coordinator for piece placement and attack detection.
//! While [`Bitboard`] handles the low-level bit manipulation, [`Board`] organizes these
//! bitboards into a coherent representation of a chess position.
//!
//! ## Representation
//!
//! Internally, a [`Board`] maintains several redundant [`Bitboard`] structures to optimize different
//! types of queries:
//! * **Role Bitboards:** Six bitboards (one for each [`Role`]: Pawn, Knight, Bishop, Rook, Queen, and King)
//!   stored in [`ByRole`].
//! * **Color Bitboards:** Two bitboards (one for each [`Color`]: White and Black) stored in [`ByColor`].
//! * **Occupied Bitboard:** A single bitboard representing the union of all pieces, used for
//!   fast collision detection and move generation.
//!
//! This design allows for $O(1)$ lookup of pieces by color or role, and extremely fast
//! calculations for complex chess logic like "find all pieces attacking the king".
//!
//! ---
//!
//! ## Movement and Captures
//!
//! The [`Board`] is designed to be persistent and functional. Methods like [`Board::mve`]
//! and [`Board::capture`] return a new, updated [`Board`] instead of mutating the current one.
//!
//! ### Example: Simple Move
//! ```
//! # use ruchess::board::Board;
//! # use ruchess::square;
//! let board = Board::new(); // Starting position
//!
//! // Move the E2 pawn to E4
//! if let Some(new_board) = board.mve(square::E2, square::E4) {
//!     assert!(new_board.is_occupied(square::E4));
//!     assert!(!new_board.is_occupied(square::E2));
//! }
//! ```
//!
//! ---
//!
//! ## Attack Detection
//!
//! A core responsibility of the [`Board`] is to track which squares are under attack. This is
//! essential for determining the legality of moves (e.g., you cannot move into check) and
//! for move generation.
//!
//! ```
//! # use ruchess::board::Board;
//! # use ruchess::color::Color;
//! # use ruchess::square;
//! let board = Board::new();
//!
//! // In the starting position, E2 is attacked by several White pieces.
//! // We can check if Black is attacking E2:
//! let is_attacked_by_black = board.is_attacked(square::E2, Color::White);
//! assert!(!is_attacked_by_black);
//! ```
//!
//! ## Invariants
//!
//! Every [`Board`] must maintain the following internal invariants:
//! * **Disjoint Roles:** No single square can be marked in more than one role bitboard.
//! * **Disjoint Colors:** No square can be both White and Black.
//! * **Occupancy Parity:** The `occupied` bitboard must exactly match the union of all role bitboards
//!   and also the union of both color bitboards.

use std::fmt::Display;

use crate::{
    attacks::ATTACKS,
    bitboard::Bitboard,
    color::{ByColor, Color},
    piece::Piece,
    role::{ByRole, Role},
    square::Square,
};

/// Represents a chess board with pieces tracked by bitboards.
#[derive(Debug, Copy, Clone)]
pub struct Board {
    by_role: ByRole<Bitboard>,
    by_color: ByColor<Bitboard>,
    occupied: Bitboard,
}

impl Board {
    /// Creates a new [`Board`] with the standard starting position.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// assert!(board.is_occupied(square::A1)); // White Rook
    /// assert!(board.is_occupied(square::E2)); // White Pawn
    /// ```
    pub fn new() -> Self {
        Self {
            by_role: ByRole {
                pawn: Bitboard(0x00FF0000_0000FF00),
                rook: Bitboard(0x81000000_00000081),
                knight: Bitboard(0x42000000_00000042),
                bishop: Bitboard(0x24000000_00000024),
                queen: Bitboard(0x08000000_00000008),
                king: Bitboard(0x10000000_00000010),
            },
            by_color: ByColor {
                white: Bitboard(0x00000000_0000FFFF),
                black: Bitboard(0xFFFF0000_00000000),
            },
            occupied: Bitboard(0xFFFF0000_0000FFFF),
        }
    }

    /// An empty [`Board`] with no pieces.
    pub const EMPTY: Board = Board {
        by_role: ByRole {
            pawn: Bitboard::EMPTY,
            rook: Bitboard::EMPTY,
            knight: Bitboard::EMPTY,
            bishop: Bitboard::EMPTY,
            queen: Bitboard::EMPTY,
            king: Bitboard::EMPTY,
        },
        by_color: ByColor {
            white: Bitboard::EMPTY,
            black: Bitboard::EMPTY,
        },
        occupied: Bitboard::EMPTY,
    };

    /// Moves a piece from `orig` to `dest`.
    ///
    /// Returns `None` if `dest` is occupied or if there is no piece at `orig`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// let next = board.mve(square::E2, square::E4).unwrap();
    /// assert!(next.is_occupied(square::E4));
    /// assert!(!next.is_occupied(square::E2));
    ///
    /// // Cannot move to occupied square
    /// assert!(board.mve(square::E1, square::E2).is_none());
    /// ```
    #[must_use]
    pub fn mve(self, orig: Square, dest: Square) -> Option<Self> {
        if self.is_occupied(dest) {
            return None;
        }
        let (board, Some(piece)) = self.pop(orig) else {
            return None;
        };
        Some(board.set(dest, piece))
    }

    /// Performs a capture from `orig` to `dest`.
    ///
    /// If `capture` is `Some(sq)`, the piece at `sq` is removed (e.g. for en passant).
    /// Returns `None` if there is no piece at `orig`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let board = Board::EMPTY.set(square::E2, Piece { role: Role::Pawn, color: Color::White })
    ///                         .set(square::F3, Piece { role: Role::Knight, color: Color::Black });
    ///
    /// let next = board.capture(square::E2, square::F3, None).unwrap();
    /// assert!(next.is_occupied(square::F3));
    /// assert!(!next.is_occupied(square::E2));
    /// assert_eq!(next.piece_at(square::F3).unwrap().color, Color::White);
    /// ```
    #[must_use]
    pub fn capture(self, orig: Square, dest: Square, capture: Option<Square>) -> Option<Self> {
        let (board, Some(piece)) = self.pop(orig) else {
            return None;
        };

        let board = board.set(dest, piece);
        if let Some(sq) = capture {
            Some(board.pop(sq).0)
        } else {
            Some(board)
        }
    }

    /// Returns a new [`Board`] with the [`Square`] `sq` set to `p`.
    ///
    /// Overwrites any existing piece.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let piece = Piece { role: Role::Rook, color: Color::White };
    /// let board = Board::EMPTY.set(square::A1, piece);
    /// assert_eq!(board.piece_at(square::A1), Some(piece));
    /// ```
    #[must_use]
    pub fn set(self, sq: Square, p: Piece) -> Self {
        let (board, _) = self.pop(sq);
        Self {
            by_role: board.by_role.update(p.role, |bb| bb.set(sq)),
            by_color: board.by_color.update(p.color, |bb| bb.set(sq)),
            occupied: board.occupied.set(sq),
        }
    }

    /// Returns a new [`Board`] with piece at `sq` removed, returning it if any.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// let (next, piece) = board.pop(square::A1);
    /// assert!(piece.is_some());
    /// assert!(!next.is_occupied(square::A1));
    /// ```
    #[must_use]
    pub fn pop(self, sq: Square) -> (Self, Option<Piece>) {
        if !self.is_occupied(sq) {
            (self, None)
        } else {
            let piece = self.piece_at(sq).unwrap();
            let board = Self {
                by_role: self.by_role.update(piece.role, |bb| bb.unset(sq)),
                by_color: self.by_color.update(piece.color, |bb| bb.unset(sq)),
                occupied: self.occupied.unset(sq),
            };

            (board, Some(piece))
        }
    }

    /// Checks whether the [`Square`] `sq` is occupied.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// assert!(board.is_occupied(square::A1));
    /// assert!(!board.is_occupied(square::E4));
    /// ```
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.occupied.is_set(sq)
    }

    /// Returns a [`Bitboard`] containing all occupied squares.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.occupied().0.count_ones(), 32);
    /// ```
    pub fn occupied(&self) -> Bitboard {
        self.occupied
    }

    /// Returns the [`Piece`] at [`Square`] `sq` if any.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let board = Board::new();
    /// let piece = board.piece_at(square::A1).unwrap();
    /// assert_eq!(piece.role, Role::Rook);
    /// assert_eq!(piece.color, Color::White);
    /// ```
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.color_at(sq)
            .and_then(|c| self.role_at(sq).map(|r| Piece { role: r, color: c }))
    }

    /// Returns the [`Role`] at [`Square`] `sq` if any.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// # use ruchess::role::Role;
    /// let board = Board::new();
    /// assert_eq!(board.role_at(square::E1), Some(Role::King));
    /// ```
    pub fn role_at(&self, sq: Square) -> Option<Role> {
        self.by_role.find(|bb| bb.is_set(sq)).map(|(p, _)| p)
    }

    /// Returns the [`Color`] at [`Square`] `sq` if any.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::square;
    /// # use ruchess::color::Color;
    /// let board = Board::new();
    /// assert_eq!(board.color_at(square::A1), Some(Color::White));
    /// assert_eq!(board.color_at(square::A8), Some(Color::Black));
    /// ```
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        self.by_color.find(|bb| bb.is_set(sq)).map(|(c, _)| c)
    }

    /// Checks whether the [`Piece`] `p` exists on the board.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let board = Board::new();
    /// let white_king = Piece { role: Role::King, color: Color::White };
    /// assert!(board.has_piece(white_king));
    /// ```
    pub fn has_piece(&self, p: Piece) -> bool {
        self.bypiece(p).is_non_empty()
    }

    /// Checks whether [`Color`] `c`'s king is in check.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// let board = Board::new();
    /// assert!(!board.is_check(Color::White));
    /// ```
    pub fn is_check(&self, c: Color) -> bool {
        self.attackers(self.king(c), c.opponent()).is_non_empty()
    }

    /// Checks whether the [`Square`] `sq` belonging to [`Color`] `c` is being attacked by [`Color::opponent`].
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// assert!(!board.is_attacked(square::E2, Color::White));
    /// ```
    pub fn is_attacked(&self, sq: Square, c: Color) -> bool {
        self.attackers(sq, c.opponent()).is_non_empty()
    }

    /// Returns a [`Bitboard`] of all `attacker`-colored pieces that have `sq` in their attack range.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// let attackers = board.attackers(square::E3, Color::White);
    /// assert!(attackers.is_non_empty()); // E2 pawn, etc.
    /// ```
    pub fn attackers(&self, sq: Square, attacker: Color) -> Bitboard {
        self.bycolor(attacker)
            & (ATTACKS.rook_attacks(sq, self.occupied)
                & (self.byrole(Role::Rook) ^ self.byrole(Role::Queen))
                | ATTACKS.bishop_attacks(sq, self.occupied)
                    & (self.byrole(Role::Bishop) ^ self.byrole(Role::Queen))
                | ATTACKS.knight_attacks(sq) & self.byrole(Role::Knight)
                | ATTACKS.king_attacks(sq) & self.byrole(Role::King)
                | ATTACKS.pawn_attacks(attacker.opponent(), sq) & self.byrole(Role::Pawn))
    }

    /// Returns a [`Bitboard`] of all pawns.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.pawns().0.count_ones(), 16);
    /// ```
    pub fn pawns(&self) -> Bitboard {
        self.byrole(Role::Pawn)
    }

    /// Returns a [`Bitboard`] of all rooks.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.rooks().0.count_ones(), 4);
    /// ```
    pub fn rooks(&self) -> Bitboard {
        self.byrole(Role::Rook)
    }

    /// Returns a [`Bitboard`] of all knights.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.knights().0.count_ones(), 4);
    /// ```
    pub fn knights(&self) -> Bitboard {
        self.byrole(Role::Knight)
    }

    /// Returns a [`Bitboard`] of all bishops.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.bishops().0.count_ones(), 4);
    /// ```
    pub fn bishops(&self) -> Bitboard {
        self.byrole(Role::Bishop)
    }

    /// Returns a [`Bitboard`] of all queens.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.queens().0.count_ones(), 2);
    /// ```
    pub fn queens(&self) -> Bitboard {
        self.byrole(Role::Queen)
    }

    /// Returns the [`Square`] of the king of [`Color`] `c`.
    ///
    /// # Panics
    ///
    /// Panics if there is not exactly one king of that color.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// # use ruchess::square;
    /// let board = Board::new();
    /// assert_eq!(board.king(Color::White), square::E1);
    /// ```
    pub fn king(&self, c: Color) -> Square {
        self.bypiece(Piece {
            role: Role::King,
            color: c,
        })
        .try_into()
        .expect("there must be exactly 1 king per color")
    }

    /// Returns a [`Bitboard`] of all white pieces.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.white().0.count_ones(), 16);
    /// ```
    pub fn white(&self) -> Bitboard {
        self.bycolor(Color::White)
    }

    /// Returns a [`Bitboard`] of all black pieces.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// let board = Board::new();
    /// assert_eq!(board.black().0.count_ones(), 16);
    /// ```
    pub fn black(&self) -> Bitboard {
        self.bycolor(Color::Black)
    }

    /// Returns a [`Bitboard`] of all pieces of the given [`Piece`].
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let board = Board::new();
    /// let white_pawn = Piece { role: Role::Pawn, color: Color::White };
    /// assert_eq!(board.bypiece(white_pawn).0.count_ones(), 8);
    /// ```
    pub fn bypiece(&self, Piece { role, color }: Piece) -> Bitboard {
        self.byrole(role) & self.bycolor(color)
    }

    /// Returns a [`Bitboard`] of all pieces of the given [`Color`].
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// let board = Board::new();
    /// assert_eq!(board.bycolor(Color::White).0.count_ones(), 16);
    /// ```
    pub fn bycolor(&self, c: Color) -> Bitboard {
        *self.by_color.get(c)
    }

    fn byrole(&self, r: Role) -> Bitboard {
        *self.by_role.get(r)
    }

    /// Converts the board into an 8x8 grid of `Option<Piece>`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::role::Role;
    /// let board = Board::new();
    /// let grid = board.into_grid();
    /// assert_eq!(grid[0][0].unwrap().role, Role::Rook);
    /// ```
    pub fn into_grid(&self) -> [[Option<Piece>; 8]; 8] {
        let mut grid = [[None; 8]; 8];

        for i in 0..64 {
            let sq = Square(i as u8);
            if self.occupied.is_set(sq) {
                let (color, _color_bb) = self.by_color.find(|b| b.is_set(sq)).unwrap();
                let (role, _role_bb) = self.by_role.find(|b| b.is_set(sq)).unwrap();
                grid[i / 8][i % 8] = Some(Piece { color, role })
            }
        }

        grid
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.into_grid().iter().rev() {
            for c in row {
                match c {
                    Some(p) => write!(f, "{}", p.as_char()),
                    None => write!(f, "."),
                }?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "there must be exactly 1 king per color"]
    fn panic_when_no_kings() {
        let bb = Board {
            by_role: ByRole::from(|_| Bitboard::EMPTY),
            by_color: ByColor::from(|c| match c {
                Color::White => Bitboard::EMPTY,
                Color::Black => Bitboard::EMPTY,
            }),
            occupied: Bitboard::EMPTY,
        };

        _ = bb.king(Color::White);
    }

    #[test]
    #[should_panic = "there must be exactly 1 king per color"]
    fn panic_when_no_white_king() {
        let bb = Board {
            by_role: ByRole::from(|_| Bitboard::EMPTY),
            by_color: ByColor::from(|c| match c {
                Color::White => Bitboard::EMPTY,
                Color::Black => Bitboard::new(0x0001_0000_0000_0000),
            }),
            occupied: Bitboard::EMPTY,
        };

        _ = bb.king(Color::White);
    }

    #[test]
    #[should_panic = "there must be exactly 1 king per color"]
    fn panic_when_no_black_king() {
        let bb = Board {
            by_role: ByRole::from(|_| Bitboard::EMPTY),
            by_color: ByColor::from(|c| match c {
                Color::White => Bitboard::new(0x0001_0000_0000_0000),
                Color::Black => Bitboard::EMPTY,
            }),
            occupied: Bitboard::EMPTY,
        };

        _ = bb.king(Color::Black);
    }

    #[test]
    #[should_panic = "there must be exactly 1 king per color"]
    fn panic_when_more_than_one_white_king() {
        let bb = Board {
            by_role: ByRole::from(|_| Bitboard::EMPTY),
            by_color: ByColor::from(|c| match c {
                Color::White => Bitboard::new(0x0000_0001_0000_0001),
                Color::Black => Bitboard::new(0x0001_0000_0000_0000),
            }),
            occupied: Bitboard::EMPTY,
        };

        _ = bb.king(Color::White);
    }

    #[test]
    #[should_panic = "there must be exactly 1 king per color"]
    fn panic_when_more_than_one_black_king() {
        let bb = Board {
            by_role: ByRole::from(|_| Bitboard::EMPTY),
            by_color: ByColor::from(|c| match c {
                Color::White => Bitboard::new(0x0001_0000_0000_0000),
                Color::Black => Bitboard::new(0x0000_0001_0000_0001),
            }),
            occupied: Bitboard::EMPTY,
        };

        _ = bb.king(Color::Black);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    const ALL_ROLES: [Role; 6] = [
        Role::Pawn,
        Role::Rook,
        Role::Knight,
        Role::Bishop,
        Role::Queen,
        Role::King,
    ];
    const ALL_COLORS: [Color; 2] = [Color::White, Color::Black];

    // ── Strategies ───────────────────────────────────────────────────────

    fn sq() -> impl Strategy<Value = Square> {
        (0u8..64).prop_map(Square)
    }

    fn role() -> impl Strategy<Value = Role> {
        prop_oneof![
            Just(Role::Pawn),
            Just(Role::Rook),
            Just(Role::Knight),
            Just(Role::Bishop),
            Just(Role::Queen),
            Just(Role::King),
        ]
    }

    fn color() -> impl Strategy<Value = Color> {
        prop_oneof![Just(Color::White), Just(Color::Black)]
    }

    fn piece() -> impl Strategy<Value = Piece> {
        (role(), color()).prop_map(|(role, color)| Piece { role, color })
    }

    /// Builds a board by replaying a random sequence of `set` operations on `EMPTY`.
    ///
    /// Operations may overlap, exercising the overwrite path of `set`.
    fn random_board() -> impl Strategy<Value = Board> {
        prop::collection::vec((sq(), piece()), 0..32).prop_map(|ops| {
            let mut b = Board::EMPTY;
            for (s, p) in ops {
                b = b.set(s, p);
            }
            b
        })
    }

    /// (board, orig ∈ occupied, dest ∈ empty). Avoids rejection by drawing
    /// from the board's actual squares instead of `prop_assume!`-filtering.
    fn board_and_move() -> impl Strategy<Value = (Board, Square, Square)> {
        random_board()
            .prop_filter("need ≥1 occupied and ≥1 empty square", |b| {
                let n = b.occupied().0.count_ones();
                (1..64).contains(&n)
            })
            .prop_flat_map(|b| {
                let occ: Vec<Square> = b.occupied().into_iter().collect();
                let emp: Vec<Square> = (!b.occupied()).into_iter().collect();
                (
                    Just(b),
                    proptest::sample::select(occ),
                    proptest::sample::select(emp),
                )
            })
    }

    /// (board, orig ∈ occupied, dest ∈ occupied, orig != dest).
    fn board_and_two_occupied() -> impl Strategy<Value = (Board, Square, Square)> {
        random_board()
            .prop_filter("need ≥2 occupied squares", |b| {
                b.occupied().0.count_ones() >= 2
            })
            .prop_flat_map(|b| {
                let occ: Vec<Square> = b.occupied().into_iter().collect();
                (
                    Just(b),
                    proptest::sample::select(occ.clone()),
                    proptest::sample::select(occ),
                )
            })
            .prop_filter("orig != dest", |(_, o, d)| o != d)
    }

    /// (board, orig ∈ occupied, dest ∈ any, cap ∈ occupied) with cap distinct
    /// from both orig and dest — the en-passant-like capture shape.
    fn board_and_capture_target() -> impl Strategy<Value = (Board, Square, Square, Square)> {
        random_board()
            .prop_filter("need ≥2 occupied squares", |b| {
                b.occupied().0.count_ones() >= 2
            })
            .prop_flat_map(|b| {
                let occ: Vec<Square> = b.occupied().into_iter().collect();
                (
                    Just(b),
                    proptest::sample::select(occ.clone()),
                    sq(),
                    proptest::sample::select(occ),
                )
            })
            .prop_filter("cap != orig and cap != dest", |(_, o, d, c)| {
                c != o && c != d
            })
    }

    // ── Equivalence helpers (Board has no PartialEq) ─────────────────────

    fn pieces_equiv(a: Piece, b: Piece) -> bool {
        a.role == b.role && a.color == b.color
    }

    fn boards_equiv(a: &Board, b: &Board) -> bool {
        if a.occupied() != b.occupied() {
            return false;
        }
        for r in ALL_ROLES {
            if a.byrole(r) != b.byrole(r) {
                return false;
            }
        }
        for c in ALL_COLORS {
            if a.bycolor(c) != b.bycolor(c) {
                return false;
            }
        }
        true
    }

    // ── Invariant checker ────────────────────────────────────────────────

    fn check_invariants(b: &Board) -> Result<(), TestCaseError> {
        let white = b.bycolor(Color::White);
        let black = b.bycolor(Color::Black);
        let occ = b.occupied();

        prop_assert_eq!(occ, white | black, "occupied != white | black");
        prop_assert_eq!(white & black, Bitboard::EMPTY, "white and black overlap");

        for (i, &r1) in ALL_ROLES.iter().enumerate() {
            for &r2 in &ALL_ROLES[i + 1..] {
                prop_assert_eq!(
                    b.byrole(r1) & b.byrole(r2),
                    Bitboard::EMPTY,
                    "roles overlap"
                );
            }
        }

        let mut role_union = Bitboard::EMPTY;
        for &r in &ALL_ROLES {
            role_union |= b.byrole(r);
        }
        prop_assert_eq!(role_union, occ, "union of role bitboards != occupied");

        for i in 0u8..64 {
            let s = Square(i);
            prop_assert_eq!(
                b.piece_at(s).is_some(),
                b.is_occupied(s),
                "piece_at vs is_occupied disagree"
            );
        }

        let grid = b.into_grid();
        for i in 0u8..64 {
            let s = Square(i);
            let row = (i / 8) as usize;
            let col = (i % 8) as usize;
            match (b.piece_at(s), grid[row][col]) {
                (None, None) => {}
                (Some(p1), Some(p2)) => {
                    prop_assert!(pieces_equiv(p1, p2), "grid vs piece_at piece mismatch")
                }
                _ => prop_assert!(false, "grid vs piece_at presence mismatch"),
            }
        }
        Ok(())
    }

    // ── Layer 1: invariants on hand-built boards ─────────────────────────

    #[test]
    fn invariants_for_empty() {
        check_invariants(&Board::EMPTY).unwrap();
    }

    #[test]
    fn invariants_for_starting_position() {
        check_invariants(&Board::new()).unwrap();
    }

    proptest! {
        // ── Layer 1: invariants under random construction & mutation ────

        #[test]
        fn invariants_for_random_board(b in random_board()) {
            check_invariants(&b)?;
        }

        #[test]
        fn invariants_after_set(b in random_board(), s in sq(), p in piece()) {
            check_invariants(&b.set(s, p))?;
        }

        #[test]
        fn invariants_after_pop(b in random_board(), s in sq()) {
            let (after, _) = b.pop(s);
            check_invariants(&after)?;
        }

        #[test]
        fn invariants_after_mve(b in random_board(), orig in sq(), dest in sq()) {
            if let Some(after) = b.mve(orig, dest) {
                check_invariants(&after)?;
            }
        }

        #[test]
        fn invariants_after_capture_no_target(
            b in random_board(), orig in sq(), dest in sq(),
        ) {
            if let Some(after) = b.capture(orig, dest, None) {
                check_invariants(&after)?;
            }
        }

        #[test]
        fn invariants_after_capture_with_target(
            b in random_board(), orig in sq(), dest in sq(), cap in sq(),
        ) {
            if let Some(after) = b.capture(orig, dest, Some(cap)) {
                check_invariants(&after)?;
            }
        }

        // ── Layer 2: round-trips ────────────────────────────────────────

        #[test]
        fn empty_set_then_pop_returns_piece(s in sq(), p in piece()) {
            let (after, popped) = Board::EMPTY.set(s, p).pop(s);
            prop_assert!(boards_equiv(&after, &Board::EMPTY));
            let popped = popped.expect("pop should return the placed piece");
            prop_assert!(pieces_equiv(popped, p));
        }

        #[test]
        fn set_then_piece_at_returns_piece(b in random_board(), s in sq(), p in piece()) {
            let after = b.set(s, p);
            let got = after.piece_at(s).expect("must have piece after set");
            prop_assert!(pieces_equiv(got, p));
            prop_assert!(after.is_occupied(s));
        }

        #[test]
        fn set_is_idempotent(b in random_board(), s in sq(), p in piece()) {
            let once = b.set(s, p);
            let twice = once.set(s, p);
            prop_assert!(boards_equiv(&once, &twice));
        }

        #[test]
        fn pop_then_pop_yields_none(b in random_board(), s in sq()) {
            let (after, _) = b.pop(s);
            let (_, second) = after.pop(s);
            prop_assert!(second.is_none());
            prop_assert!(!after.is_occupied(s));
        }

        #[test]
        fn pop_empty_square_is_noop(b in random_board(), s in sq()) {
            prop_assume!(!b.is_occupied(s));
            let (after, popped) = b.pop(s);
            prop_assert!(popped.is_none());
            prop_assert!(boards_equiv(&b, &after));
        }

        #[test]
        fn pop_then_set_back_restores(b in random_board(), s in sq()) {
            prop_assume!(b.is_occupied(s));
            let (after_pop, popped) = b.pop(s);
            let p = popped.unwrap();
            let restored = after_pop.set(s, p);
            prop_assert!(boards_equiv(&b, &restored));
        }

        #[test]
        fn mve_succeeds_iff_orig_occupied_and_dest_empty(
            b in random_board(), orig in sq(), dest in sq(),
        ) {
            let ok = b.is_occupied(orig) && !b.is_occupied(dest);
            prop_assert_eq!(b.mve(orig, dest).is_some(), ok);
        }

        #[test]
        fn mve_moves_piece_and_preserves_count((b, orig, dest) in board_and_move()) {
            let original = b.piece_at(orig).unwrap();
            let after = b.mve(orig, dest).unwrap();

            prop_assert!(!after.is_occupied(orig));
            prop_assert!(after.is_occupied(dest));
            let landed = after.piece_at(dest).unwrap();
            prop_assert!(pieces_equiv(landed, original));
            prop_assert_eq!(b.occupied().0.count_ones(), after.occupied().0.count_ones());
        }

        #[test]
        fn mve_round_trip((b, orig, dest) in board_and_move()) {
            let after = b.mve(orig, dest).unwrap();
            let restored = after.mve(dest, orig).unwrap();
            prop_assert!(boards_equiv(&b, &restored));
        }

        #[test]
        fn capture_none_matches_mve((b, orig, dest) in board_and_move()) {
            let mved = b.mve(orig, dest).unwrap();
            let cap = b.capture(orig, dest, None).unwrap();
            prop_assert!(boards_equiv(&mved, &cap));
        }

        #[test]
        fn capture_overwrites_dest((b, orig, dest) in board_and_two_occupied()) {
            let original = b.piece_at(orig).unwrap();
            let after = b.capture(orig, dest, None).unwrap();
            prop_assert!(!after.is_occupied(orig));
            prop_assert!(after.is_occupied(dest));
            let landed = after.piece_at(dest).unwrap();
            prop_assert!(pieces_equiv(landed, original));
            // Captured one piece: count drops by exactly one.
            prop_assert_eq!(
                b.occupied().0.count_ones(),
                after.occupied().0.count_ones() + 1
            );
        }

        #[test]
        fn capture_with_target_removes_captured(
            (b, orig, dest, cap) in board_and_capture_target(),
        ) {
            let after = b.capture(orig, dest, Some(cap)).unwrap();
            prop_assert!(!after.is_occupied(cap));
            prop_assert!(after.is_occupied(dest));
        }
    }
}
