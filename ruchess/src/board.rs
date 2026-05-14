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
    #[must_use]
    pub fn set(self, sq: Square, p: Piece) -> Self {
        let (board, piece) = self.pop(sq);
        match piece {
            Some(_) => Self {
                by_role: board.by_role.update(p.role, |bb| bb.set(sq)),
                by_color: board.by_color.update(p.color, |bb| bb.set(sq)),
                occupied: board.occupied,
            },
            None => Self {
                by_role: board.by_role.update(p.role, |bb| bb.set(sq)),
                by_color: board.by_color.update(p.color, |bb| bb.set(sq)),
                occupied: board.occupied.set(sq),
            },
        }
    }

    /// Returns a new [`Board`] with piece at `sq` removed, returning it if any.
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
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.occupied.is_set(sq)
    }

    /// Returns a [`Bitboard`] containing all occupied squares.
    pub fn occupied(&self) -> Bitboard {
        self.occupied
    }

    /// Returns the [`Piece`] at [`Square`] `sq` if any.
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.color_at(sq)
            .and_then(|c| self.role_at(sq).map(|r| Piece { role: r, color: c }))
    }

    /// Returns the [`Role`] at [`Square`] `sq` if any.
    pub fn role_at(&self, sq: Square) -> Option<Role> {
        self.by_role.find(|bb| bb.is_set(sq)).map(|(p, _)| p)
    }

    /// Returns the [`Color`] at [`Square`] `sq` if any.
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        self.by_color.find(|bb| bb.is_set(sq)).map(|(c, _)| c)
    }

    /// Checks whether the [`Piece`] `p` exists on the board.
    pub fn has_piece(&self, p: Piece) -> bool {
        self.bypiece(p).is_non_empty()
    }

    /// Checks whether [`Color`] `c`'s king is in check.
    pub fn is_check(&self, c: Color) -> bool {
        self.attackers(self.king(c), c.opponent()).is_non_empty()
    }

    /// Checks whether the [`Square`] `sq` belonging to [`Color`] `c` is being attacked by [`Color::opponent`].
    pub fn is_attacked(&self, sq: Square, c: Color) -> bool {
        self.attackers(sq, c.opponent()).is_non_empty()
    }

    /// Returns a [`Bitboard`] of all `attacker`-colored pieces that have `sq` in their attack range.
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
    pub fn pawns(&self) -> Bitboard {
        self.byrole(Role::Pawn)
    }

    /// Returns a [`Bitboard`] of all rooks.
    pub fn rooks(&self) -> Bitboard {
        self.byrole(Role::Rook)
    }

    /// Returns a [`Bitboard`] of all knights.
    pub fn knights(&self) -> Bitboard {
        self.byrole(Role::Knight)
    }

    /// Returns a [`Bitboard`] of all bishops.
    pub fn bishops(&self) -> Bitboard {
        self.byrole(Role::Bishop)
    }

    /// Returns a [`Bitboard`] of all queens.
    pub fn queens(&self) -> Bitboard {
        self.byrole(Role::Queen)
    }

    /// Returns the [`Square`] of the king of [`Color`] `c`.
    ///
    /// # Panics
    ///
    /// Panics if there is not exactly one king of that color.
    pub fn king(&self, c: Color) -> Square {
        self.bypiece(Piece {
            role: Role::King,
            color: c,
        })
        .try_into()
        .expect("only 1 king per color")
    }

    /// Returns a [`Bitboard`] of all white pieces.
    pub fn white(&self) -> Bitboard {
        self.bycolor(Color::White)
    }

    /// Returns a [`Bitboard`] of all black pieces.
    pub fn black(&self) -> Bitboard {
        self.bycolor(Color::Black)
    }

    /// Returns a [`Bitboard`] of all pieces of the given [`Piece`].
    pub fn bypiece(&self, Piece { role, color }: Piece) -> Bitboard {
        self.byrole(role) & self.bycolor(color)
    }

    /// Returns a [`Bitboard`] of all pieces of the given [`Color`].
    pub fn bycolor(&self, c: Color) -> Bitboard {
        *self.by_color.get(c)
    }

    fn byrole(&self, r: Role) -> Bitboard {
        *self.by_role.get(r)
    }

    /// Converts the board into an 8x8 grid of `Option<Piece>`.
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
