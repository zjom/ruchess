//! # Squares
//!
//! A [`Square`] is one of the 64 cells on a chess board, identified by a
//! single `u8` index using the same Little-Endian Rank-File (LERF) mapping as
//! [`Bitboard`](crate::bitboard::Bitboard): `A1 == 0`, `H1 == 7`, …, `H8 == 63`.
//!
//! The 64 named constants ([`A1`] through [`H8`]) cover every square. Squares
//! parse from algebraic notation (`"e4"`, `"E4"`) via [`FromStr`].
//!
//! ```
//! # use ruchess::square::{self, Square};
//! # use std::str::FromStr;
//! assert_eq!(Square::from_str("e4").unwrap(), square::E4);
//! assert_eq!(square::A1.file().as_str(), "A");
//! assert_eq!(square::A1.rank().as_str(), "1");
//! ```

use crate::{file::File, rank::Rank};

use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::color::Color;

/// A single square on the chess board, addressed by a 0..=63 index.
///
/// The inner `u8` is the LERF index: `A1 = 0`, `H1 = 7`, …, `H8 = 63`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Square(pub u8);
impl Square {
    /// Constructs a [`Square`] from a 0-based index.
    ///
    /// # Panics
    /// Panics if `idx >= 64`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::square::{self, Square};
    /// assert_eq!(Square::new(0), square::A1);
    /// assert_eq!(Square::new(63), square::H8);
    /// ```
    pub const fn new(idx: u8) -> Square {
        assert!(idx < 64);
        Square(idx)
    }

    /// Constructs a [`Square`] from a [`File`] and [`Rank`].
    ///
    /// # Example
    /// ```
    /// # use ruchess::square::{self, Square};
    /// # use ruchess::file::File;
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Square::from_file_and_rank(File::E, Rank::Fourth), square::E4);
    /// ```
    pub const fn from_file_and_rank(file: File, rank: Rank) -> Self {
        Self::new(rank.as_u8() * 8 + file.as_u8())
    }

    /// Returns the [`Rank`] of this square.
    ///
    /// # Example
    /// ```
    /// # use ruchess::square;
    /// # use ruchess::rank::Rank;
    /// assert_eq!(square::E4.rank(), Rank::Fourth);
    /// ```
    pub fn rank(&self) -> Rank {
        Rank::new((self.0 / 8) as u32)
    }

    /// Returns the [`File`] of this square.
    ///
    /// # Example
    /// ```
    /// # use ruchess::square;
    /// # use ruchess::file::File;
    /// assert_eq!(square::E4.file(), File::E);
    /// ```
    pub fn file(&self) -> File {
        File::new((self.0 % 8) as u32)
    }

    /// Returns the absolute distance, in ranks, between `self` and `other`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::square;
    /// assert_eq!(square::A1.ydist(square::A3), 2);
    /// assert_eq!(square::A3.ydist(square::A1), 2);
    /// assert_eq!(square::A1.ydist(square::H1), 0);
    /// ```
    pub fn ydist(self, other: Square) -> u8 {
        let (a, b) = (self.rank().as_u8(), other.rank().as_u8());

        a.wrapping_sub(b).min(b.wrapping_sub(a))
    }

    /// Returns the square one rank "behind" this one, from `color`'s point of
    /// view, or `None` if this square is already on `color`'s back rank.
    ///
    /// "Behind" means the previous rank along the direction of advance:
    /// rank-1 for White, rank+1 for Black.
    ///
    /// # Example
    /// ```
    /// # use ruchess::square;
    /// # use ruchess::color::Color;
    /// assert_eq!(square::E4.prev_rank(Color::White), Some(square::E3));
    /// assert_eq!(square::E5.prev_rank(Color::Black), Some(square::E6));
    /// assert_eq!(square::E1.prev_rank(Color::White), None);
    /// ```
    pub fn prev_rank(self, color: Color) -> Option<Square> {
        match color {
            Color::White => {
                if self.rank() == Rank::First {
                    None
                } else {
                    Some(Square::from_file_and_rank(
                        self.file(),
                        Rank::new((self.rank().as_u8() - 1) as u32),
                    ))
                }
            }
            Color::Black => {
                if self.rank() == Rank::Eighth {
                    None
                } else {
                    Some(Square::from_file_and_rank(
                        self.file(),
                        Rank::new((self.rank().as_u8() + 1) as u32),
                    ))
                }
            }
        }
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file().as_str();
        let rank = self.rank().as_str();
        write!(f, "{}{}", file, rank)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file().as_str();
        let rank = self.rank().as_str();
        write!(f, "{}{}", file, rank)
    }
}

/// Error returned when a string cannot be parsed as a [`Square`].
///
/// The wrapped `String` is the offending input, preserved verbatim for
/// diagnostics.
#[derive(Debug, PartialEq)]
pub struct ParseSquareError(pub String);
impl Display for ParseSquareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid square notation: `{}`", self.0)
    }
}
impl Error for ParseSquareError {}

impl FromStr for Square {
    type Err = ParseSquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.trim().to_ascii_uppercase();
        let p = norm.as_bytes();
        match p {
            [b'A'..=b'H', b'1'..=b'8'] => {
                let rank = p[0] - b'A';
                let file = p[1] - b'1';
                Ok(Square(file * 8 + rank))
            }
            _ => Err(ParseSquareError(s.into())),
        }
    }
}

/// The A1 square.
pub const A1: Square = Square(0);
/// The B1 square.
pub const B1: Square = Square(1);
/// The C1 square.
pub const C1: Square = Square(2);
/// The D1 square.
pub const D1: Square = Square(3);
/// The E1 square.
pub const E1: Square = Square(4);
/// The F1 square.
pub const F1: Square = Square(5);
/// The G1 square.
pub const G1: Square = Square(6);
/// The H1 square.
pub const H1: Square = Square(7);
/// The A2 square.
pub const A2: Square = Square(8);
/// The B2 square.
pub const B2: Square = Square(9);
/// The C2 square.
pub const C2: Square = Square(10);
/// The D2 square.
pub const D2: Square = Square(11);
/// The E2 square.
pub const E2: Square = Square(12);
/// The F2 square.
pub const F2: Square = Square(13);
/// The G2 square.
pub const G2: Square = Square(14);
/// The H2 square.
pub const H2: Square = Square(15);
/// The A3 square.
pub const A3: Square = Square(16);
/// The B3 square.
pub const B3: Square = Square(17);
/// The C3 square.
pub const C3: Square = Square(18);
/// The D3 square.
pub const D3: Square = Square(19);
/// The E3 square.
pub const E3: Square = Square(20);
/// The F3 square.
pub const F3: Square = Square(21);
/// The G3 square.
pub const G3: Square = Square(22);
/// The H3 square.
pub const H3: Square = Square(23);
/// The A4 square.
pub const A4: Square = Square(24);
/// The B4 square.
pub const B4: Square = Square(25);
/// The C4 square.
pub const C4: Square = Square(26);
/// The D4 square.
pub const D4: Square = Square(27);
/// The E4 square.
pub const E4: Square = Square(28);
/// The F4 square.
pub const F4: Square = Square(29);
/// The G4 square.
pub const G4: Square = Square(30);
/// The H4 square.
pub const H4: Square = Square(31);
/// The A5 square.
pub const A5: Square = Square(32);
/// The B5 square.
pub const B5: Square = Square(33);
/// The C5 square.
pub const C5: Square = Square(34);
/// The D5 square.
pub const D5: Square = Square(35);
/// The E5 square.
pub const E5: Square = Square(36);
/// The F5 square.
pub const F5: Square = Square(37);
/// The G5 square.
pub const G5: Square = Square(38);
/// The H5 square.
pub const H5: Square = Square(39);
/// The A6 square.
pub const A6: Square = Square(40);
/// The B6 square.
pub const B6: Square = Square(41);
/// The C6 square.
pub const C6: Square = Square(42);
/// The D6 square.
pub const D6: Square = Square(43);
/// The E6 square.
pub const E6: Square = Square(44);
/// The F6 square.
pub const F6: Square = Square(45);
/// The G6 square.
pub const G6: Square = Square(46);
/// The H6 square.
pub const H6: Square = Square(47);
/// The A7 square.
pub const A7: Square = Square(48);
/// The B7 square.
pub const B7: Square = Square(49);
/// The C7 square.
pub const C7: Square = Square(50);
/// The D7 square.
pub const D7: Square = Square(51);
/// The E7 square.
pub const E7: Square = Square(52);
/// The F7 square.
pub const F7: Square = Square(53);
/// The G7 square.
pub const G7: Square = Square(54);
/// The H7 square.
pub const H7: Square = Square(55);
/// The A8 square.
pub const A8: Square = Square(56);
/// The B8 square.
pub const B8: Square = Square(57);
/// The C8 square.
pub const C8: Square = Square(58);
/// The D8 square.
pub const D8: Square = Square(59);
/// The E8 square.
pub const E8: Square = Square(60);
/// The F8 square.
pub const F8: Square = Square(61);
/// The G8 square.
pub const G8: Square = Square(62);
/// The H8 square.
pub const H8: Square = Square(63);
