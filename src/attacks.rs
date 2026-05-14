//! # Precomputed Attack Tables
//!
//! Move generation hangs off [`ATTACKS`], a single lazily-initialized table
//! holding every piece's attack pattern from every square. For knights,
//! kings, and pawns the table is a plain array; for sliding pieces (rooks,
//! bishops, queens) it uses the classic **magic bitboard** technique: the
//! relevant-blocker mask is multiplied by a magic constant (see
//! [`Magic`](crate::magic::Magic)) to produce an index into a shared array
//! of pre-baked attack sets.
//!
//! The table is ~780 KB. Initialization runs once on first access and is
//! boxed onto the heap to avoid blowing the default test-thread stack.
//!
//! ## Example
//! ```
//! # use ruchess::attacks::ATTACKS;
//! # use ruchess::bitboard::Bitboard;
//! # use ruchess::square;
//! // A knight on the corner attacks exactly two squares.
//! let bb = ATTACKS.knight_attacks(square::A1);
//! assert_eq!(bb.0.count_ones(), 2);
//! ```

use crate::{bitboard::Bitboard, color::Color, magic::Magic, square::Square};
use lazy_static::lazy_static;

lazy_static! {
    /// Global, lazily-initialized attack tables.
    ///
    /// Boxed onto the heap (around 780 KB) and initialized on first access.
    pub static ref ATTACKS: Box<Attacks> = Attacks::new();
}

/// Bundle of precomputed attack tables for every piece type.
///
/// Prefer [`ATTACKS`] over constructing this directly — there is no reason
/// to have more than one copy.
#[derive(Clone, Copy)]
pub struct Attacks {
    /// `ranks[i]` is the 8-bit mask for rank `i`, used during initialization.
    pub ranks: [u64; 8],
    /// `files[i]` is the 8-bit mask for file `i`, used during initialization.
    pub files: [u64; 8],
    /// `between[a][b]` is the set of squares strictly between `a` and `b`
    /// along a rook or bishop ray, or `0` if they are not aligned.
    pub between: [[u64; 64]; 64],
    /// `rays[a][b]` is the full line through `a` and `b` (rook or bishop
    /// ray, including both endpoints), or `0` if not aligned.
    pub rays: [[u64; 64]; 64],

    /// Shared backing array for sliding-piece magics. Indexed via
    /// [`Magic::rook_index`] / [`Magic::bishop_index`].
    pub attacks: [u64; 88772],
    /// `knight_attacks[sq]` is the attack set of a knight on `sq`.
    pub knight_attacks: [u64; 64],
    /// `king_attacks[sq]` is the attack set of a king on `sq`.
    pub king_attacks: [u64; 64],
    /// `white_pawn_attacks[sq]` is the diagonal attack set of a white pawn on `sq`.
    pub white_pawn_attacks: [u64; 64],
    /// `black_pawn_attacks[sq]` is the diagonal attack set of a black pawn on `sq`.
    pub black_pawn_attacks: [u64; 64],
}

#[allow(clippy::new_without_default)]
impl Attacks {
    fn new() -> Box<Self> {
        // All fields are `[u64; N]`, so the all-zero bit pattern is a valid
        // `Attacks`. Allocate directly on the heap to avoid the ~780 KB stack
        // temporary that would otherwise blow the default 2 MB test-thread stack.
        let mut a: Box<Self> = unsafe { Box::<Self>::new_zeroed().assume_init() };
        a.initialize();
        a
    }

    /// Returns the bitboard of squares a rook on `sq` attacks given the
    /// `occupied` bitboard of blockers.
    pub fn rook_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        let m = &Magic::ROOK[sq.0 as usize];
        Bitboard(self.attacks[m.rook_index(occupied.0)])
    }

    /// Returns the bitboard of squares a bishop on `sq` attacks given the
    /// `occupied` bitboard of blockers.
    pub fn bishop_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        let m = &Magic::BISHOP[sq.0 as usize];
        Bitboard(self.attacks[m.bishop_index(occupied.0)])
    }

    /// Returns the squares a pawn of the given `color` on `sq` attacks
    /// (diagonally forward). Pushes are not included.
    pub fn pawn_attacks(&self, color: Color, sq: Square) -> Bitboard {
        Bitboard(match color {
            Color::White => self.white_pawn_attacks[sq.0 as usize],
            Color::Black => self.black_pawn_attacks[sq.0 as usize],
        })
    }

    /// Returns the squares a king on `sq` attacks (the eight neighbors).
    pub fn king_attacks(&self, sq: Square) -> Bitboard {
        Bitboard(self.king_attacks[sq.0 as usize])
    }

    /// Returns the squares a knight on `sq` attacks.
    pub fn knight_attacks(&self, sq: Square) -> Bitboard {
        Bitboard(self.knight_attacks[sq.0 as usize])
    }

    fn initialize(&mut self) {
        for i in 0..8 {
            self.ranks[i] = 0xffu64 << (i * 8);
            self.ranks[i] = 0x0101010101010101u64 << i;
        }

        for sq in 0i32..64 {
            self.knight_attacks[sq as usize] = sliding_attacks(sq, u64::MAX, &KNIGHT_DELTAS);
            self.king_attacks[sq as usize] = sliding_attacks(sq, u64::MAX, &KING_DELTAS);
            self.white_pawn_attacks[sq as usize] =
                sliding_attacks(sq, u64::MAX, &WHITE_PAWN_DELTAS);
            self.black_pawn_attacks[sq as usize] =
                sliding_attacks(sq, u64::MAX, &BLACK_PAWN_DELTAS);

            init_magics(
                &mut self.attacks,
                sq,
                &Magic::ROOK[sq as usize],
                12,
                &ROOK_DELTAS,
            );
            init_magics(
                &mut self.attacks,
                sq,
                &Magic::BISHOP[sq as usize],
                9,
                &BISHOP_DELTAS,
            );
        }

        for a in 0i32..64 {
            for b in 0i32..64 {
                if sliding_attacks(a, 0, &ROOK_DELTAS) & (1u64 << b) != 0 {
                    self.between[a as usize][b as usize] =
                        sliding_attacks(a, 1u64 << b, &ROOK_DELTAS)
                            & sliding_attacks(b, 1u64 << a, &ROOK_DELTAS);
                    self.rays[a as usize][b as usize] = (1u64 << a)
                        | (1u64 << b)
                        | sliding_attacks(a, 0, &ROOK_DELTAS) & sliding_attacks(b, 0, &ROOK_DELTAS);
                } else if sliding_attacks(a, 0, &BISHOP_DELTAS) & (1u64 << b) != 0 {
                    self.between[a as usize][b as usize] =
                        sliding_attacks(a, 1u64 << b, &BISHOP_DELTAS)
                            & sliding_attacks(b, 1u64 << a, &BISHOP_DELTAS);
                    self.rays[a as usize][b as usize] = (1u64 << a)
                        | (1u64 << b)
                        | sliding_attacks(a, 0, &BISHOP_DELTAS)
                            & sliding_attacks(b, 0, &BISHOP_DELTAS);
                }
            }
        }
    }
}

const KNIGHT_DELTAS: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
const BISHOP_DELTAS: [i32; 4] = [7, -7, 9, -9];
const ROOK_DELTAS: [i32; 4] = [1, -1, 8, -8];
const KING_DELTAS: [i32; 8] = [1, 7, 8, 9, -1, -7, -8, -9];
const WHITE_PAWN_DELTAS: [i32; 2] = [7, 9];
const BLACK_PAWN_DELTAS: [i32; 2] = [-7, -9];

/// Chebyshev distance between two square indices, treating off-board
/// indices as if they were on an infinite grid. Used to detect when a
/// `delta` step has wrapped around the board edge.
fn distance(a: i32, b: i32) -> i32 {
    let file = |s: i32| s & 7;
    let rank = |s: i32| s >> 3;
    (file(a) - file(b)).abs().max((rank(a) - rank(b)).abs())
}

/// Computes the set of squares reachable from `square` by sliding along
/// each of `deltas`, stopping when an `occupied` bit is hit or the slide
/// would wrap off the board.
fn sliding_attacks(square: i32, occupied: u64, deltas: &[i32]) -> u64 {
    let mut attacks = 0u64;
    for &delta in deltas {
        let mut sq = square;
        loop {
            sq += delta;
            let oob = !(0..64).contains(&sq) || distance(sq, sq - delta) > 2;
            if oob {
                break;
            }
            attacks |= 1u64 << sq;
            if occupied & (1u64 << sq) != 0 {
                break;
            }
        }
    }
    attacks
}

/// Fills the shared `attacks` array for one (square, magic) pair by
/// enumerating every subset of `magic.mask`, computing the sliding attack
/// for that occupancy, and storing it at the magic-indexed slot.
fn init_magics(attacks: &mut [u64; 88772], square: i32, magic: &Magic, shift: u32, deltas: &[i32]) {
    let mut subset = 0u64;
    loop {
        let attack = sliding_attacks(square, subset, deltas);
        let idx = ((magic.factor.wrapping_mul(subset)) >> (64 - shift)) as usize + magic.offset;
        attacks[idx] = attack;

        subset = subset.wrapping_sub(magic.mask) & magic.mask;
        if subset == 0 {
            break;
        }
    }
}
