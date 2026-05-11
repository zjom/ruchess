use std::fmt::Display;

use crate::{
    attacks::ATTACKS,
    bitboard::Bitboard,
    color::{ByColor, Color},
    piece::Piece,
    role::{ByRole, Role},
    square::Square,
};

#[derive(Debug)]
pub struct Board {
    by_role: ByRole<Bitboard>,
    by_color: ByColor<Bitboard>,
    occupied: Bitboard,
}

impl Board {
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

    /// returns a new [`Board`] with the [`Square`] `sq` set to `p`
    ///
    /// overwrites any existing piece
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

    /// returns a new [`Board`] with piece at `sq` removed, returning it if any
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

    /// whether a square is occupied
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.occupied.is_set(sq)
    }

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.color_at(sq)
            .and_then(|c| self.role_at(sq).map(|r| Piece { role: r, color: c }))
    }

    pub fn role_at(&self, sq: Square) -> Option<Role> {
        self.by_role.find(|bb| bb.is_set(sq)).map(|(p, _)| p)
    }
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        self.by_color.find(|bb| bb.is_set(sq)).map(|(c, _)| c)
    }

    /// whether a piece exists
    pub fn has_piece(&self, p: Piece) -> bool {
        self.bypiece(p).is_non_empty()
    }

    /// whether color `c`'s king in check
    pub fn is_check(&self, c: Color) -> bool {
        self.attackers(self.king_sq(c), c.opponent()).is_non_empty()
    }

    /// whether the square `sq` belonging to color `c` being attacked by `c`'s opponent
    pub fn is_attacked(&self, sq: Square, c: Color) -> bool {
        self.attackers(sq, c.opponent()).is_non_empty()
    }

    /// returns a `Bitboard` of all `attacker`-colored pieces that have `sq` in their attack range.
    pub fn attackers(&self, sq: Square, attacker: Color) -> Bitboard {
        self.bycolor(attacker)
            & (self.rook_attacks(sq) & (self.byrole(Role::Rook) ^ self.byrole(Role::Queen))
                | self.bishop_attacks(sq) & (self.byrole(Role::Bishop) ^ self.byrole(Role::Queen))
                | self.knight_attacks(sq) & self.byrole(Role::Knight)
                | self.king_attacks(sq) & self.byrole(Role::King)
                | self.pawn_attacks(attacker.opponent(), sq) & self.byrole(Role::Pawn))
    }

    fn king_sq(&self, c: Color) -> Square {
        self.bypiece(Piece {
            role: Role::King,
            color: c,
        })
        .try_into()
        .expect("only 1 king per color")
    }

    fn rook_attacks(&self, sq: Square) -> Bitboard {
        Bitboard(ATTACKS.rook_attacks(sq.0 as usize, self.occupied.0))
    }

    fn bishop_attacks(&self, sq: Square) -> Bitboard {
        Bitboard(ATTACKS.bishop_attacks(sq.0 as usize, self.occupied.0))
    }

    fn pawn_attacks(&self, color: Color, sq: Square) -> Bitboard {
        Bitboard(match color {
            Color::White => ATTACKS.white_pawn_attacks[sq.0 as usize],
            Color::Black => ATTACKS.black_pawn_attacks[sq.0 as usize],
        })
    }
    fn king_attacks(&self, sq: Square) -> Bitboard {
        Bitboard(ATTACKS.king_attacks[sq.0 as usize])
    }

    fn knight_attacks(&self, sq: Square) -> Bitboard {
        Bitboard(ATTACKS.knight_attacks[sq.0 as usize])
    }

    fn bypiece(&self, p: Piece) -> Bitboard {
        self.byrole(p.role) & self.bycolor(p.color)
    }
    fn bycolor(&self, c: Color) -> Bitboard {
        *self.by_color.get(c)
    }

    fn byrole(&self, r: Role) -> Bitboard {
        *self.by_role.get(r)
    }

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
        for row in self.into_grid() {
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
