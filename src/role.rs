//! # Chess Piece Roles
//!
//! This module defines the types used to represent the different roles or types of pieces
//! in a chess game. It includes the comprehensive [`Role`] enum, the [`PromotableRole`]
//! subset for pawn promotions, and the [`ByRole`] container for storing data indexed by role.
//!
//! ---
//!
//! ## Example: Piece Roles and ASCII
//! ```
//! # use ruchess::role::Role;
//! let king = Role::King;
//! assert_eq!(king.as_ascii(), 'k');
//! ```

use std::{error::Error, fmt::Display, str::FromStr};

/// Represents the type of a chess piece.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Role {
    /// A Pawn.
    Pawn,
    /// A Rook.
    Rook,
    /// A Knight.
    Knight,
    /// A Bishop.
    Bishop,
    /// A Queen.
    Queen,
    /// A King.
    King,
}

/// Represents the subset of piece roles that a pawn can be promoted to.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PromotableRole {
    /// A Rook.
    Rook,
    /// A Knight.
    Knight,
    /// A Bishop.
    Bishop,
    /// A Queen.
    Queen,
}

impl Role {
    /// Returns the lowercase ASCII representation of the role.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::role::Role;
    ///
    /// assert_eq!(Role::Pawn.as_ascii(), 'p');
    /// assert_eq!(Role::Rook.as_ascii(), 'r');
    /// assert_eq!(Role::Knight.as_ascii(), 'n');
    /// assert_eq!(Role::Bishop.as_ascii(), 'b');
    /// assert_eq!(Role::Queen.as_ascii(), 'q');
    /// assert_eq!(Role::King.as_ascii(), 'k');
    /// ```
    pub fn as_ascii(&self) -> char {
        match self {
            Role::Pawn => 'p',
            Role::Rook => 'r',
            Role::Knight => 'n',
            Role::Bishop => 'b',
            Role::Queen => 'q',
            Role::King => 'k',
        }
    }

    /// Parses a role from a lowercase ASCII character.
    ///
    /// Returns a [`ParseRoleError`] if the character does not correspond to a valid promotable role.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::Role;
    /// assert_eq!(Role::from_ascii('n'), Ok(Role::Knight));
    /// assert!(Role::from_ascii('p').is_err());
    /// ```
    pub fn from_ascii(c: char) -> Result<Self, ParseRoleError> {
        match c {
            'r' => Ok(Role::Rook),
            'n' => Ok(Role::Knight),
            'b' => Ok(Role::Bishop),
            'q' => Ok(Role::Queen),
            _ => Err(ParseRoleError(c.to_string())),
        }
    }
}

impl TryFrom<char> for Role {
    type Error = ParseRoleError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Role::from_ascii(value)
    }
}

impl TryFrom<u8> for Role {
    type Error = ParseRoleError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Role::from_ascii(value.into())
    }
}

impl PromotableRole {
    /// An array of all possible promotable roles.
    pub const ROLES: [PromotableRole; 4] = [Self::Rook, Self::Knight, Self::Bishop, Self::Queen];

    /// Returns the lowercase ASCII representation of the promotable role.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::PromotableRole;
    /// assert_eq!(PromotableRole::Queen.as_ascii(), 'q');
    /// ```
    pub fn as_ascii(&self) -> char {
        match self {
            PromotableRole::Rook => 'r',
            PromotableRole::Knight => 'n',
            PromotableRole::Bishop => 'b',
            PromotableRole::Queen => 'q',
        }
    }

    /// Parses a promotable role from a lowercase ASCII character.
    ///
    /// Returns a [`ParseRoleError`] if the character does not correspond to a valid promotable role.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::PromotableRole;
    /// assert_eq!(PromotableRole::from_ascii('n'), Ok(PromotableRole::Knight));
    /// assert!(PromotableRole::from_ascii('p').is_err());
    /// ```
    pub fn from_ascii(c: char) -> Result<Self, ParseRoleError> {
        match c {
            'r' => Ok(PromotableRole::Rook),
            'n' => Ok(PromotableRole::Knight),
            'b' => Ok(PromotableRole::Bishop),
            'q' => Ok(PromotableRole::Queen),
            _ => Err(ParseRoleError(c.to_string())),
        }
    }
}

impl TryFrom<char> for PromotableRole {
    type Error = ParseRoleError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        PromotableRole::from_ascii(value)
    }
}

impl TryFrom<u8> for PromotableRole {
    type Error = ParseRoleError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        PromotableRole::from_ascii(value.into())
    }
}

/// Error returned when parsing a role from a string or character fails.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseRoleError(String);

impl Display for ParseRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid role: {}", self.0)
    }
}
impl Error for ParseRoleError {}

impl FromStr for PromotableRole {
    type Err = ParseRoleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseRoleError(s.to_string()));
        };

        Self::from_ascii(s.chars().next().unwrap())
    }
}

/// A container that stores a value of type `T` for each [`Role`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ByRole<T> {
    /// Value for [`Role::Pawn`].
    pub pawn: T,
    /// Value for [`Role::Rook`].
    pub rook: T,
    /// Value for [`Role::Knight`].
    pub knight: T,
    /// Value for [`Role::Bishop`].
    pub bishop: T,
    /// Value for [`Role::Queen`].
    pub queen: T,
    /// Value for [`Role::King`].
    pub king: T,
}

impl<T> ByRole<T> {
    /// Creates a new `ByRole` container with the given values.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::ByRole;
    /// let container = ByRole::new(1, 3, 3, 5, 9, 0);
    /// assert_eq!(container.pawn, 1);
    /// assert_eq!(container.queen, 9);
    /// ```
    #[inline]
    pub fn new(pawn: T, knight: T, bishop: T, rook: T, queen: T, king: T) -> Self {
        Self {
            pawn,
            rook,
            knight,
            bishop,
            queen,
            king,
        }
    }

    /// Returns a reference to the value associated with the given [`Role`].
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// let container = ByRole::new(1, 2, 3, 4, 5, 6);
    /// assert_eq!(*container.get(Role::Pawn), 1);
    /// assert_eq!(*container.get(Role::King), 6);
    /// ```
    #[inline]
    pub fn get(&self, r: Role) -> &T {
        match r {
            Role::Pawn => &self.pawn,
            Role::Knight => &self.knight,
            Role::Bishop => &self.bishop,
            Role::Rook => &self.rook,
            Role::Queen => &self.queen,
            Role::King => &self.king,
        }
    }

    /// Sets the value for the given [`Role`] and returns the updated container.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// let container = ByRole::new(0, 0, 0, 0, 0, 0).set(Role::Queen, 9);
    /// assert_eq!(container.queen, 9);
    /// ```
    #[must_use]
    #[inline]
    pub fn set(self, r: Role, value: T) -> Self {
        match r {
            Role::Pawn => Self {
                pawn: value,
                ..self
            },
            Role::Knight => Self {
                knight: value,
                ..self
            },
            Role::Bishop => Self {
                bishop: value,
                ..self
            },
            Role::Rook => Self {
                rook: value,
                ..self
            },
            Role::Queen => Self {
                queen: value,
                ..self
            },
            Role::King => Self {
                king: value,
                ..self
            },
        }
    }

    /// Updates the value for the given [`Role`] using a function and returns the updated container.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// let container = ByRole::new(1, 1, 1, 1, 1, 1).update(Role::Rook, |v| v + 4);
    /// assert_eq!(container.rook, 5);
    /// ```
    #[must_use]
    #[inline]
    pub fn update<F>(self, r: Role, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        match r {
            Role::Pawn => Self {
                pawn: f(self.pawn),
                ..self
            },
            Role::Knight => Self {
                knight: f(self.knight),
                ..self
            },
            Role::Bishop => Self {
                bishop: f(self.bishop),
                ..self
            },
            Role::Rook => Self {
                rook: f(self.rook),
                ..self
            },
            Role::Queen => Self {
                queen: f(self.queen),
                ..self
            },
            Role::King => Self {
                king: f(self.king),
                ..self
            },
        }
    }

    /// Updates the value for the given [`Role`] using a function in place.
    ///
    /// # Example
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// let mut container = ByRole::new(1, 1, 1, 1, 1, 1);
    /// container.update_mut(Role::Rook, |v| *v += 4);
    /// assert_eq!(container.rook, 5);
    /// ```
    #[inline]
    pub fn update_mut<F>(&mut self, r: Role, mut f: F)
    where
        F: FnMut(&mut T),
    {
        match r {
            Role::Pawn => f(&mut self.pawn),
            Role::Knight => f(&mut self.knight),
            Role::Bishop => f(&mut self.bishop),
            Role::Rook => f(&mut self.rook),
            Role::Queen => f(&mut self.queen),
            Role::King => f(&mut self.king),
        };
    }

    /// Executes some side effect `f` for each item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// let values = ByRole { pawn: 1, rook: 5, knight: 3, bishop: 3, queen: 9, king: 0 };
    /// let mut sum = 0;
    /// values.foreach(|score| sum += score);
    /// assert_eq!(sum, 21);
    /// ```
    #[inline]
    pub fn foreach<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        f(&self.pawn);
        f(&self.rook);
        f(&self.knight);
        f(&self.bishop);
        f(&self.queen);
        f(&self.king);
    }

    /// Creates a new instance of `ByRole` by applying `f` to each item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// // Standard material values doubled
    /// let values = ByRole { pawn: 1, rook: 5, knight: 3, bishop: 3, queen: 9, king: 0 };
    /// let doubled = values.map(|v| v * 2);
    /// assert_eq!(doubled.pawn, 2);
    /// assert_eq!(doubled.rook, 10);
    /// assert_eq!(doubled.queen, 18);
    /// ```
    #[must_use]
    #[inline]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(&T) -> T,
    {
        Self {
            pawn: f(&self.pawn),
            rook: f(&self.rook),
            knight: f(&self.knight),
            bishop: f(&self.bishop),
            queen: f(&self.queen),
            king: f(&self.king),
        }
    }

    /// Creates a new instance of `ByRole` by generator `f`.
    ///
    /// The generator receives the [`Role`] variant and returns the value for that role.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// // Build a ByRole from standard material point values
    /// let values = ByRole::from(|role| match role {
    ///     Role::Pawn   => 1,
    ///     Role::Knight => 3,
    ///     Role::Bishop => 3,
    ///     Role::Rook   => 5,
    ///     Role::Queen  => 9,
    ///     Role::King   => 0,
    /// });
    /// assert_eq!(values.pawn, 1);
    /// assert_eq!(values.queen, 9);
    /// ```
    #[must_use]
    #[inline]
    pub fn from<F>(f: F) -> Self
    where
        F: Fn(Role) -> T,
    {
        Self {
            pawn: f(Role::Pawn),
            rook: f(Role::Rook),
            knight: f(Role::Knight),
            bishop: f(Role::Bishop),
            queen: f(Role::Queen),
            king: f(Role::King),
        }
    }

    /// Returns the first [`Role`] where `f` returns `true`,
    /// or `None` if no role matches. Roles are checked in the order:
    /// pawn → rook → knight → bishop → queen → king.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::role::{ByRole, Role};
    /// let counts = ByRole { pawn: 0, rook: 0, knight: 0, bishop: 0, queen: 1, king: 1 };
    ///
    /// // Finds the first role with a non-zero count (queen comes before king)
    /// assert_eq!(counts.find(|c| *c > 0), Some(Role::Queen);
    ///
    /// // Returns None when no role matches
    /// let empty = ByRole { pawn: 0, rook: 0, knight: 0, bishop: 0, queen: 0, king: 0 };
    /// assert_eq!(empty.find(|c| *c > 0), None);
    /// ```
    #[inline]
    pub fn find<F>(&self, f: F) -> Option<Role>
    where
        F: Fn(&T) -> bool,
    {
        if f(&self.pawn) {
            Some(Role::Pawn)
        } else if f(&self.rook) {
            Some(Role::Rook)
        } else if f(&self.knight) {
            Some(Role::Knight)
        } else if f(&self.bishop) {
            Some(Role::Bishop)
        } else if f(&self.queen) {
            Some(Role::Queen)
        } else if f(&self.king) {
            Some(Role::King)
        } else {
            None
        }
    }
}
