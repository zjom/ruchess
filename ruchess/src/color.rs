#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(self) -> Color {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ByColor<T> {
    white: T,
    black: T,
}

impl<T> ByColor<T> {
    pub fn new(white: T, black: T) -> Self {
        Self { white, black }
    }
    pub fn get(&self, c: Color) -> &T {
        match c {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

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
