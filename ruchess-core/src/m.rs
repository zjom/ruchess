use std::fmt::Display;
use std::str::FromStr;

use crate::error::ParseSquareError;
use crate::{Bitboard, Board, Color, Piece, Role, Square};

#[derive(Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl FromStr for Move {
    type Err = ParseSquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 4 {
            return Err(ParseSquareError(s.into()));
        }
        let from = s[..2].parse::<Square>()?;
        let to = s[2..4].parse::<Square>()?;
        Ok(Move { from, to })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

pub fn pseudo_legal_moves(
    piece: Piece,
    from: &Square,
    board: &Board,
    en_passant: Option<Square>,
) -> Bitboard {
    let Piece(role, color) = piece;
    let occupied = board.occupied();
    let own = board.bycolor(color);

    let candidates = match role {
        Role::Pawn => pawn_moves(
            color,
            from,
            occupied,
            board.bycolor(color.opponent()),
            en_passant,
        ),
        Role::Knight => knight_moves(from),
        Role::Bishop => bishop_moves(from, occupied),
        Role::Rook => rook_moves(from, occupied),
        Role::Queen => bishop_moves(from, occupied) | rook_moves(from, occupied),
        Role::King => king_moves(from),
    };

    // Cannot land on own pieces
    Bitboard(candidates.0 & !own.0)
}

pub fn pawn_moves(
    color: Color,
    from: &Square,
    occupied: Bitboard,
    enemies: Bitboard,
    en_passant: Option<Square>,
) -> Bitboard {
    let idx = from.0 as i32;
    let file = idx % 8;
    let rank = idx / 8;
    let mut targets = Bitboard(0);

    let (dir, start_rank): (i32, i32) = match color {
        Color::White => (1, 1),
        Color::Black => (-1, 6),
    };

    let push_idx = idx + dir * 8;

    // Single and double push (only if path is clear)
    if (0..64).contains(&push_idx) {
        let sq = Square(push_idx as u8);
        if !occupied.contains(&sq) {
            targets = targets.set(&sq);
            if rank == start_rank {
                let push2_idx = idx + dir * 16;
                let sq2 = Square(push2_idx as u8);
                if !occupied.contains(&sq2) {
                    targets = targets.set(&sq2);
                }
            }
        }
    }

    // Diagonal captures
    for df in [-1i32, 1] {
        let new_file = file + df;
        if !(0..8).contains(&new_file) {
            continue;
        }
        let cap_idx = push_idx + df;
        if !(0..64).contains(&cap_idx) {
            continue;
        }
        let sq = Square(cap_idx as u8);
        if enemies.contains(&sq) || en_passant == Some(sq) {
            targets = targets.set(&sq);
        }
    }

    targets
}

pub fn knight_moves(from: &Square) -> Bitboard {
    let idx = from.0 as i32;
    let file = idx % 8;
    let rank = idx / 8;
    let mut targets = Bitboard(0);

    for (df, dr) in [
        (1, 2),
        (-1, 2),
        (2, 1),
        (-2, 1),
        (1, -2),
        (-1, -2),
        (2, -1),
        (-2, -1),
    ] {
        let new_file = file + df;
        let new_rank = rank + dr;
        if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
            targets = targets.set(&Square((new_rank * 8 + new_file) as u8));
        }
    }

    targets
}

pub fn king_moves(from: &Square) -> Bitboard {
    let idx = from.0 as i32;
    let file = idx % 8;
    let rank = idx / 8;
    let mut targets = Bitboard(0);

    for df in -1i32..=1 {
        for dr in -1i32..=1 {
            if df == 0 && dr == 0 {
                continue;
            }
            let new_file = file + df;
            let new_rank = rank + dr;
            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                targets = targets.set(&Square((new_rank * 8 + new_file) as u8));
            }
        }
    }

    targets
}

pub fn rook_moves(from: &Square, occupied: Bitboard) -> Bitboard {
    let idx = from.0 as i32;
    let file = idx % 8;
    let rank = idx / 8;
    let mut targets = Bitboard(0);

    for r in (rank + 1)..8 {
        let sq = Square((r * 8 + file) as u8);
        targets = targets.set(&sq);
        if occupied.contains(&sq) {
            break;
        }
    }
    for r in (0..rank).rev() {
        let sq = Square((r * 8 + file) as u8);
        targets = targets.set(&sq);
        if occupied.contains(&sq) {
            break;
        }
    }
    for f in (file + 1)..8 {
        let sq = Square((rank * 8 + f) as u8);
        targets = targets.set(&sq);
        if occupied.contains(&sq) {
            break;
        }
    }
    for f in (0..file).rev() {
        let sq = Square((rank * 8 + f) as u8);
        targets = targets.set(&sq);
        if occupied.contains(&sq) {
            break;
        }
    }

    targets
}

pub fn bishop_moves(from: &Square, occupied: Bitboard) -> Bitboard {
    let idx = from.0 as i32;
    let file = idx % 8;
    let rank = idx / 8;
    let mut targets = Bitboard(0);

    for (df, dr) in [(1i32, 1i32), (1, -1), (-1, 1), (-1, -1)] {
        let mut f = file + df;
        let mut r = rank + dr;
        while (0..8).contains(&f) && (0..8).contains(&r) {
            let sq = Square((r * 8 + f) as u8);
            targets = targets.set(&sq);
            if occupied.contains(&sq) {
                break;
            }
            f += df;
            r += dr;
        }
    }

    targets
}
