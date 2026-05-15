//! # UCI Move Notation
//!
//! [Universal Chess Interface](https://backscattering.de/chess/uci/) long
//! algebraic move strings — `e2e4`, `g1f3`, `e7e8q` for promotions. This
//! module provides [`Uci`], a stripped-down move record that captures just
//! the origin, destination, and optional promotion, plus parsing and
//! formatting between [`Uci`] and `&str`.
//!
//! ```
//! # use ruchess::uci::Uci;
//! # use std::str::FromStr;
//! let m = Uci::from_str("e7e8q").unwrap();
//! // Squares render in uppercase; the promotion suffix stays lowercase.
//! assert_eq!(m.uci(), "E7E8q");
//! ```

use std::{error::Error, fmt::Display, str::FromStr};

use crate::{
    mve::Move,
    role::{ParseRoleError, PromotableRole},
    square::{ParseSquareError, Square},
};

/// A move expressed in UCI long algebraic notation.
///
/// `Uci` is a thin record: it knows the origin and destination squares plus
/// an optional promotion role, but carries none of the board context that
/// [`Move`] does. Use [`Uci::from_move`] (or `Uci::from(m)`) to project a
/// [`Move`] down to its UCI representation, and [`FromStr`] to parse a UCI
/// string.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uci {
    /// The origin square.
    pub orig: Square,
    /// The destination square.
    pub dest: Square,
    /// The role the pawn promotes to, if this is a promotion.
    pub promotion: Option<PromotableRole>,
}

impl Uci {
    /// Projects a [`Move`] down to its UCI fields (origin, destination,
    /// promotion).
    ///
    /// # Example
    /// ```
    /// # use ruchess::uci::Uci;
    /// # use ruchess::mve::Move;
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// # use ruchess::square;
    /// let m = Move::quiet(
    ///     Piece { role: Role::Pawn, color: Color::White },
    ///     square::E2,
    ///     square::E4,
    /// );
    /// let u = Uci::from_move(m);
    /// assert_eq!(u.orig, square::E2);
    /// assert_eq!(u.dest, square::E4);
    /// assert!(u.promotion.is_none());
    /// ```
    pub fn from_move(m: Move) -> Self {
        Self {
            orig: m.orig,
            dest: m.dest,
            promotion: m.promotion,
        }
    }

    /// Returns the canonical UCI string: `"<orig><dest>"` for ordinary moves,
    /// `"<orig><dest><promo>"` for promotions.
    ///
    /// Squares render in uppercase (e.g. `"E2E4"`); the promotion suffix is
    /// the lowercase ASCII role letter.
    ///
    /// # Example
    /// ```
    /// # use ruchess::uci::Uci;
    /// # use ruchess::square;
    /// # use ruchess::role::PromotableRole;
    /// let push = Uci { orig: square::E2, dest: square::E4, promotion: None };
    /// assert_eq!(push.uci(), "E2E4");
    ///
    /// let prom = Uci {
    ///     orig: square::E7,
    ///     dest: square::E8,
    ///     promotion: Some(PromotableRole::Queen),
    /// };
    /// assert_eq!(prom.uci(), "E7E8q");
    /// ```
    pub fn uci(&self) -> String {
        match self.promotion {
            Some(r) => format!("{}{}{}", self.orig, self.dest, r.as_ascii()),
            None => format!("{}{}", self.orig, self.dest,),
        }
    }

    fn from_str(s: &str) -> Result<Self, UciFormatError> {
        match s.len() {
            4 | 5 => {
                let orig = Square::from_str(&s[0..2])?;
                let dest = Square::from_str(&s[2..4])?;
                let promotion = match s.len() {
                    5 => Some(PromotableRole::from_str(&s[4..5])?),
                    _ => None,
                };

                Ok(Self {
                    orig,
                    dest,
                    promotion,
                })
            }
            _ => Err(UciFormatError::InvalidLength(s.to_string())),
        }
    }
}

impl From<Move> for Uci {
    fn from(value: Move) -> Self {
        Self::from_move(value)
    }
}

impl std::fmt::Display for Uci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.uci())
    }
}

impl FromStr for Uci {
    type Err = UciFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

/// Error returned when parsing a UCI string fails.
#[derive(Debug)]
pub enum UciFormatError {
    /// The string was not 4 or 5 characters long.
    InvalidLength(String),
    /// One of the origin or destination square components failed to parse.
    ParseSquareError(ParseSquareError),
    /// The promotion character was not a valid promotable role.
    ParsePromotionError(ParseRoleError),
}

impl Display for UciFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength(input) => writeln!(f, "invalid uci format: {}", input),
            Self::ParseSquareError(error) => writeln!(f, "invalid uci format: {}", error),
            Self::ParsePromotionError(error) => writeln!(f, "invalid uci format: {}", error),
        }
    }
}
impl Error for UciFormatError {}

impl From<ParseSquareError> for UciFormatError {
    fn from(value: ParseSquareError) -> Self {
        Self::ParseSquareError(value)
    }
}
impl From<ParseRoleError> for UciFormatError {
    fn from(value: ParseRoleError) -> Self {
        Self::ParsePromotionError(value)
    }
}
