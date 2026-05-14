//! # Ranks
//!
//! A [`Rank`] is one of the eight horizontal rows on a chess board, numbered
//! `1` through `8` from White's perspective. Ranks are stored as a `u8`
//! discriminant (`First = 0`, `Eighth = 7`) and can be converted to a
//! [`Bitboard`](crate::bitboard::Bitboard) mask via [`Rank::MASKS`] or
//! `Bitboard::from(rank)`.

/// One of the eight ranks on a chess board, numbered from White's perspective.
///
/// The discriminant is the 0-based rank index: `Rank::First as u8 == 0`,
/// `Rank::Eighth as u8 == 7`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Rank {
    /// Rank 1 — White's back rank.
    First = 0,
    /// Rank 2 — White's pawn rank.
    Second,
    /// Rank 3.
    Third,
    /// Rank 4.
    Fourth,
    /// Rank 5.
    Fifth,
    /// Rank 6.
    Sixth,
    /// Rank 7 — Black's pawn rank.
    Seventh,
    /// Rank 8 — Black's back rank.
    Eighth,
}
impl Rank {
    /// Constructs a [`Rank`] from a 0-based index.
    ///
    /// # Panics
    /// Panics if `idx >= 8`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Rank::new(0), Rank::First);
    /// assert_eq!(Rank::new(7), Rank::Eighth);
    /// ```
    #[inline]
    pub const fn new(idx: u32) -> Rank {
        assert!(idx < 8);
        unsafe { Rank::new_unchecked(idx) }
    }

    /// Constructs a [`Rank`] from a 0-based index without bounds checking.
    ///
    /// # Safety
    ///
    /// Function must be called with an index < 8.
    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Rank {
        debug_assert!(index < 8);
        unsafe { std::mem::transmute(index as u8) }
    }

    /// Bitboard masks indexed by rank. `MASKS[rank as usize]` is a `u64`
    /// with every square on that rank set.
    pub const MASKS: [u64; 8] = [
        0x0000_0000_0000_00FF, // Rank 1
        0x0000_0000_0000_FF00, // Rank 2
        0x0000_0000_00FF_0000, // Rank 3
        0x0000_0000_FF00_0000, // Rank 4
        0x0000_00FF_0000_0000, // Rank 5
        0x0000_FF00_0000_0000, // Rank 6
        0x00FF_0000_0000_0000, // Rank 7
        0xFF00_0000_0000_0000, // Rank 8
    ];

    /// Returns the underlying 0-based rank index.
    ///
    /// # Example
    /// ```
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Rank::First.as_u8(), 0);
    /// assert_eq!(Rank::Eighth.as_u8(), 7);
    /// ```
    #[inline]
    pub const fn as_u8(self) -> u8 {
        // Safety: self is repr u8
        unsafe { std::mem::transmute(self) }
    }
    /// Returns the ASCII digit (`'1'`–`'8'`) for this rank.
    ///
    /// # Example
    /// ```
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Rank::First.as_char(), '1');
    /// assert_eq!(Rank::Eighth.as_char(), '8');
    /// ```
    pub fn as_char(&self) -> char {
        match self {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eighth => '8',
        }
    }

    /// Returns the ASCII digit for this rank as a string slice.
    ///
    /// # Example
    /// ```
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Rank::First.as_str(), "1");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::First => "1",
            Rank::Second => "2",
            Rank::Third => "3",
            Rank::Fourth => "4",
            Rank::Fifth => "5",
            Rank::Sixth => "6",
            Rank::Seventh => "7",
            Rank::Eighth => "8",
        }
    }

    /// Parses a rank from a single ASCII byte in the range `'1'..='8'`.
    ///
    /// Returns `None` if `value` is not a digit between `'1'` and `'8'`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Rank::from_byte(b'1'), Some(Rank::First));
    /// assert_eq!(Rank::from_byte(b'8'), Some(Rank::Eighth));
    /// assert_eq!(Rank::from_byte(b'9'), None);
    /// ```
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            b'1' => Some(Rank::First),
            b'2' => Some(Rank::Second),
            b'3' => Some(Rank::Third),
            b'4' => Some(Rank::Fourth),
            b'5' => Some(Rank::Fifth),
            b'6' => Some(Rank::Sixth),
            b'7' => Some(Rank::Seventh),
            b'8' => Some(Rank::Eighth),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank={}", self.as_str())
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank={}", self.as_str())
    }
}

/// Error returned when a string cannot be parsed as a [`Rank`].
///
/// The wrapped `String` is the offending input, preserved verbatim for
/// diagnostics.
#[derive(Debug, PartialEq)]
pub struct ParseRankError(pub String);
impl std::fmt::Display for ParseRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid rank: `{}`", self.0)
    }
}
impl std::error::Error for ParseRankError {}

impl std::str::FromStr for Rank {
    type Err = ParseRankError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.as_bytes();
        if p.len() != 1 {
            return Err(ParseRankError(s.to_string()));
        }

        Self::from_byte(p[0]).ok_or(ParseRankError(s.to_string()))
    }
}
