//! # Files
//!
//! A [`File`] is one of the eight vertical columns on a chess board, labeled
//! `A` through `H`. Files are stored as a `u8` discriminant (`A = 0`,
//! `H = 7`) and can be converted to a [`Bitboard`](crate::bitboard::Bitboard)
//! mask via [`File::MASKS`] or `Bitboard::from(file)`.

/// One of the eight files on a chess board, `A` through `H`.
///
/// The discriminant is the file index from White's perspective:
/// `File::A as u8 == 0`, `File::H as u8 == 7`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum File {
    /// The A file.
    A = 0,
    /// The B file.
    B,
    /// The C file.
    C,
    /// The D file.
    D,
    /// The E file.
    E,
    /// The F file.
    F,
    /// The G file.
    G,
    /// The H file.
    H,
}

impl File {
    /// Constructs a [`File`] from a 0-based index.
    ///
    /// # Panics
    /// Panics if `idx >= 8`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::file::File;
    /// assert_eq!(File::new(0), File::A);
    /// assert_eq!(File::new(7), File::H);
    /// ```
    #[inline]
    pub const fn new(idx: u32) -> File {
        assert!(idx < 8);
        unsafe { File::new_unchecked(idx) }
    }

    /// Constructs a [`File`] from a 0-based index without bounds checking.
    ///
    /// # Safety
    ///
    /// Function must be called with an index < 8.
    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> File {
        debug_assert!(index < 8);
        unsafe { std::mem::transmute(index as u8) }
    }

    /// Bitboard masks indexed by file. `MASKS[file as usize]` is a `u64`
    /// with every square on that file set.
    pub const MASKS: [u64; 8] = [
        0x0101_0101_0101_0101, // File A
        0x0202_0202_0202_0202, // File B
        0x0404_0404_0404_0404, // File C
        0x0808_0808_0808_0808, // File D
        0x1010_1010_1010_1010, // File E
        0x2020_2020_2020_2020, // File F
        0x4040_4040_4040_4040, // File G
        0x8080_8080_8080_8080, // File H
    ];

    /// Returns the underlying 0-based file index.
    ///
    /// # Example
    /// ```
    /// # use ruchess::file::File;
    /// assert_eq!(File::A.as_u8(), 0);
    /// assert_eq!(File::H.as_u8(), 7);
    /// ```
    #[inline]
    pub const fn as_u8(self) -> u8 {
        // Safety: self is repr u8
        unsafe { std::mem::transmute(self) }
    }

    /// Returns the uppercase ASCII letter for this file.
    ///
    /// # Example
    /// ```
    /// # use ruchess::file::File;
    /// assert_eq!(File::A.as_char(), 'A');
    /// assert_eq!(File::H.as_char(), 'H');
    /// ```
    pub fn as_char(&self) -> char {
        match self {
            File::A => 'A',
            File::B => 'B',
            File::C => 'C',
            File::D => 'D',
            File::E => 'E',
            File::F => 'F',
            File::G => 'G',
            File::H => 'H',
        }
    }

    /// Returns the uppercase ASCII letter for this file as a string slice.
    ///
    /// # Example
    /// ```
    /// # use ruchess::file::File;
    /// assert_eq!(File::A.as_str(), "A");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            File::A => "A",
            File::B => "B",
            File::C => "C",
            File::D => "D",
            File::E => "E",
            File::F => "F",
            File::G => "G",
            File::H => "H",
        }
    }

    /// Parses a file from a single ASCII byte. Accepts both upper- and lower-case.
    ///
    /// Returns `None` if `value` is not in the range `'A'..='H'` / `'a'..='h'`.
    ///
    /// # Example
    /// ```
    /// # use ruchess::file::File;
    /// assert_eq!(File::from_byte(b'a'), Some(File::A));
    /// assert_eq!(File::from_byte(b'H'), Some(File::H));
    /// assert_eq!(File::from_byte(b'1'), None);
    /// ```
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            b'A' | b'a' => Some(File::A),
            b'B' | b'b' => Some(File::B),
            b'C' | b'c' => Some(File::C),
            b'D' | b'd' => Some(File::D),
            b'E' | b'e' => Some(File::E),
            b'F' | b'f' => Some(File::F),
            b'G' | b'g' => Some(File::G),
            b'H' | b'h' => Some(File::H),
            _ => None,
        }
    }
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File={}", self.as_str())
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File={}", self.as_str())
    }
}

/// Error returned when a string cannot be parsed as a [`File`].
///
/// The wrapped `String` is the offending input, preserved verbatim for
/// diagnostics.
#[derive(Debug, PartialEq)]
pub struct ParseFileError(pub String);
impl std::fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid file: `{}`", self.0)
    }
}
impl std::error::Error for ParseFileError {}
impl std::str::FromStr for File {
    type Err = ParseFileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.as_bytes();
        if p.len() != 1 {
            return Err(ParseFileError(s.to_string()));
        }

        Self::from_byte(p[0]).ok_or(ParseFileError(s.to_string()))
    }
}
