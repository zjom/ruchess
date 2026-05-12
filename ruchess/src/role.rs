#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Role {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub enum PromotableRole {
    Rook,
    Knight,
    Bishop,
    Queen,
}

impl PromotableRole {
    pub const ROLES: [PromotableRole; 4] = [Self::Rook, Self::Knight, Self::Bishop, Self::Queen];
}

impl Role {
    /// Returns ascii representation of role.
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
}

#[derive(Debug, Clone, Copy)]
pub struct ByRole<T> {
    pub(crate) pawn: T,
    pub(crate) rook: T,
    pub(crate) knight: T,
    pub(crate) bishop: T,
    pub(crate) queen: T,
    pub(crate) king: T,
}

impl<T> ByRole<T> {
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

    /// Creates a new instance of `ByRole` with updated value.
    #[must_use]
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

    /// Creates a new instance of `ByRole` by applying `f`.
    #[must_use]
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

    /// Returns the first `(Role, &T)` pair where `f` returns `true`,
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
    /// assert_eq!(counts.find(|c| *c > 0), Some((Role::Queen, &1)));
    ///
    /// // Returns None when no role matches
    /// let empty = ByRole { pawn: 0, rook: 0, knight: 0, bishop: 0, queen: 0, king: 0 };
    /// assert_eq!(empty.find(|c| *c > 0), None);
    /// ```
    pub fn find<F>(&self, f: F) -> Option<(Role, &T)>
    where
        F: Fn(&T) -> bool,
    {
        if f(&self.pawn) {
            Some((Role::Pawn, &self.pawn))
        } else if f(&self.rook) {
            Some((Role::Rook, &self.rook))
        } else if f(&self.knight) {
            Some((Role::Knight, &self.knight))
        } else if f(&self.bishop) {
            Some((Role::Bishop, &self.bishop))
        } else if f(&self.queen) {
            Some((Role::Queen, &self.queen))
        } else if f(&self.king) {
            Some((Role::King, &self.king))
        } else {
            None
        }
    }
}
