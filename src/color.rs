//! # Colors and Color-Indexed Containers
//!
//! This module provides the [`Color`] enum to represent the two sides in a chess game
//! (White and Black), along with the [`ByColor`] container for storing data associated
//! with each side.
//!
//! ### Key Features
//! - **Perspective-Aware Ranks:** Easily get the back rank, second rank, etc., for a specific color.
//! - **Castling Support:** Quickly find the home square of a rook based on color and side.
//! - **Utility Containers:** [`ByColor<T>`] provides a type-safe way to store side-specific data
//!   (like bitboards, scores, or king positions).
//!
//! ---
//!
//! ## Example: Basic Color Operations
//! ```
//! # use ruchess::color::Color;
//! let side = Color::White;
//! assert_eq!(side.opponent(), Color::Black);
//! assert_eq!(side.back_rank(), ruchess::rank::Rank::First);
//! ```

use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::rank::Rank;
use crate::side::Side;
use crate::square::{self, Square};

/// Represents one of the two players in a chess game.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    /// The player who moves first.
    White,
    /// The player who moves second.
    Black,
}

impl Color {
    /// Returns the opposite color.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::Color;
    /// assert_eq!(Color::White.opponent(), Color::Black);
    /// assert_eq!(Color::Black.opponent(), Color::White);
    /// ```
    pub fn opponent(self) -> Color {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    /// The rook's home square for this color and castling side.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::Color;
    /// # use ruchess::side::Side;
    /// # use ruchess::square;
    /// assert_eq!(Color::White.castle_square(Side::King), square::H1);
    /// assert_eq!(Color::Black.castle_square(Side::Queen), square::A8);
    /// ```
    pub fn castle_square(self, side: Side) -> Square {
        match (self, side) {
            (Color::White, Side::King) => square::H1,
            (Color::White, Side::Queen) => square::A1,
            (Color::Black, Side::King) => square::H8,
            (Color::Black, Side::Queen) => square::A8,
        }
    }

    /// Returns the back rank (where major pieces start) for this color.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::Color;
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Color::White.back_rank(), Rank::First);
    /// assert_eq!(Color::Black.back_rank(), Rank::Eighth);
    /// ```
    pub fn back_rank(self) -> Rank {
        match self {
            Self::White => Rank::First,
            Self::Black => Rank::Eighth,
        }
    }

    /// Returns the second rank (where pawns start) for this color.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::Color;
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Color::White.second_rank(), Rank::Second);
    /// assert_eq!(Color::Black.second_rank(), Rank::Seventh);
    /// ```
    pub fn second_rank(self) -> Rank {
        match self {
            Self::White => Rank::Second,
            Self::Black => Rank::Seventh,
        }
    }

    /// Returns the fourth rank relative to this color's perspective.
    ///
    /// This is the rank pawns reach after a double-step from their starting position.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::Color;
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Color::White.fourth_rank(), Rank::Fourth);
    /// assert_eq!(Color::Black.fourth_rank(), Rank::Fifth);
    /// ```
    pub fn fourth_rank(self) -> Rank {
        match self {
            Self::White => Rank::Fourth,
            Self::Black => Rank::Fifth,
        }
    }

    /// Returns the seventh rank (the rank before promotion) for this color.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::Color;
    /// # use ruchess::rank::Rank;
    /// assert_eq!(Color::White.seventh_rank(), Rank::Seventh);
    /// assert_eq!(Color::Black.seventh_rank(), Rank::Second);
    /// ```
    pub fn seventh_rank(self) -> Rank {
        match self {
            Self::White => Rank::Seventh,
            Self::Black => Rank::Second,
        }
    }

    /// Returns the singular name of the color as a lowercase string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::White => "white",
            Self::Black => "black",
        }
    }
}

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowered = s.to_lowercase();
        match lowered.as_str() {
            "w" | "white" => Ok(Self::White),
            "b" | "black" => Ok(Self::Black),
            _ => Err(ParseColorError(lowered)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseColorError(String);
impl Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid color string: {}", self.0)
    }
}
impl Error for ParseColorError {}

/// A container that stores a value of type `T` for each [`Color`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ByColor<T> {
    /// The value associated with [`Color::White`].
    pub white: T,
    /// The value associated with [`Color::Black`].
    pub black: T,
}

impl<T> ByColor<T> {
    /// Creates a new `ByColor` container with the given values.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::ByColor;
    /// let container = ByColor::new(10, 20);
    /// assert_eq!(container.white, 10);
    /// assert_eq!(container.black, 20);
    /// ```
    pub fn new(white: T, black: T) -> Self {
        Self { white, black }
    }

    /// Returns a reference to the value associated with the given [`Color`].
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::{ByColor, Color};
    /// let container = ByColor::new("W", "B");
    /// assert_eq!(*container.get(Color::White), "W");
    /// ```
    pub fn get(&self, c: Color) -> &T {
        match c {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    /// Sets the value for the given [`Color`] and returns the updated container.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::{ByColor, Color};
    /// let container = ByColor::new(0, 0).set(Color::White, 5);
    /// assert_eq!(container.white, 5);
    /// ```
    #[must_use]
    pub fn set(self, c: Color, value: T) -> Self {
        match c {
            Color::White => Self {
                white: value,
                ..self
            },
            Color::Black => Self {
                black: value,
                ..self
            },
        }
    }

    /// Updates the value for the given [`Color`] using a function and returns the updated container.
    ///
    /// # Example
    /// ```
    /// # use ruchess::color::{ByColor, Color};
    /// let container = ByColor::new(10, 20).update(Color::Black, |v| v + 5);
    /// assert_eq!(container.black, 25);
    /// ```
    #[must_use]
    pub fn update<F>(self, c: Color, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        match c {
            Color::White => Self {
                white: f(self.white),
                ..self
            },
            Color::Black => Self {
                black: f(self.black),
                ..self
            },
        }
    }

    /// Executes some side effect `f` for each item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::color::ByColor;
    /// let scores = ByColor { white: 3, black: 7 };
    /// let mut sum = 0;
    /// scores.foreach(|score| sum += score);
    /// assert_eq!(sum, 10);
    /// ```
    pub fn foreach<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        f(&self.white);
        f(&self.black);
    }

    /// Creates a new instance of `ByColor` by applying `f` to each item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::color::{ByColor, Color};
    /// let scores = ByColor { white: 3, black: 7 };
    /// let doubled = scores.map(|score| score * 2);
    /// assert_eq!(doubled.white, 6);
    /// assert_eq!(doubled.black, 14);
    /// ```
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(&T) -> T,
    {
        Self {
            white: f(&self.white),
            black: f(&self.black),
        }
    }

    /// Creates a new instance of `ByColor` by generator `f`.
    ///
    /// The generator receives the [`Color`] variant and returns the value for that side.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::color::{ByColor, Color};
    /// let labels = ByColor::from(|color| format!("{color:?} player"));
    /// assert_eq!(labels.white, "White player");
    /// assert_eq!(labels.black, "Black player");
    /// ```
    pub fn from<F>(f: F) -> Self
    where
        F: Fn(Color) -> T,
    {
        Self {
            white: f(Color::White),
            black: f(Color::Black),
        }
    }

    /// Returns the first `(Color, &T)` pair where `f` returns `true`,
    /// or `None` if neither side matches. White is checked before black.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::color::{ByColor, Color};
    /// // Finds white when white matches
    /// let scores = ByColor { white: 10, black: 3 };
    /// assert_eq!(scores.find(|s| *s > 5), Some((Color::White, &10)));
    ///
    /// // Falls through to black when only black matches
    /// let scores = ByColor { white: 1, black: 8 };
    /// assert_eq!(scores.find(|s| *s > 5), Some((Color::Black, &8)));
    ///
    /// // Returns None when neither side matches
    /// let scores = ByColor { white: 1, black: 2 };
    /// assert_eq!(scores.find(|s| *s > 5), None);
    /// ```
    pub fn find<F>(&self, f: F) -> Option<(Color, &T)>
    where
        F: Fn(&T) -> bool,
    {
        if f(&self.white) {
            Some((Color::White, &self.white))
        } else if f(&self.black) {
            Some((Color::Black, &self.black))
        } else {
            None
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
