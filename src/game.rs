//! # Game
//!
//! [`Game`] is the top-level handle for playing a chess game. It wraps a
//! [`Position`] with a turn counter ([`Ply`]) and the terminal [`Outcome`],
//! if any.
//!
//! Each call to [`Game::mve`] returns a new game with the move applied. After
//! every move the resulting position is re-evaluated for checkmate, draw by
//! insufficient material, threefold repetition, fifty-move rule, and
//! stalemate, and the [`Outcome`] is set accordingly.
//!
//! ## Example
//! ```
//! # use ruchess::game::Game;
//! # use ruchess::square;
//! # use ruchess::uci::Uci;
//! let game = Game::new();
//! assert!(game.outcome().is_none());
//!
//! let after_e4 = game.mve(&Uci{orig: square::E2, dest: square::E4, promotion: None}).unwrap();
//! assert!(after_e4.position().board().is_occupied(square::E4));
//! ```

use std::fmt::Display;

use crate::{
    bitboard::Bitboard,
    board::Board,
    color::Color,
    outcome::{DrawReason, Outcome},
    position::Position,
    uci::Uci,
};

/// A complete chess game: the current [`Position`], the turn counter, and
/// the terminal [`Outcome`] if one has been reached.
#[derive(Debug, Default, Clone)]
pub struct Game {
    position: Position,
    outcome: Option<Outcome>,
}

impl Game {
    /// Starts a new game from the standard starting position, ply 0, no
    /// outcome yet.
    ///
    /// # Example
    /// ```
    /// # use ruchess::game::Game;
    /// # use ruchess::color::Color;
    /// let g = Game::new();
    /// assert_eq!(g.position().color(), Color::White);
    /// assert!(g.outcome().is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            outcome: None,
        }
    }

    /// Plays the legal move from `orig` to `dest`. Returns the resulting
    /// game, or `None` if no legal move connects those squares.
    ///
    /// On success, the turn counter advances by one ply and the new
    /// position is evaluated for any terminal outcome.
    ///
    /// # Example
    /// ```
    /// # use ruchess::game::Game;
    /// # use ruchess::square;
    /// # use ruchess::uci::Uci;
    /// let m = Uci{orig: square::E2, dest: square::E4, promotion: None};
    /// let g = Game::new().mve(&m).unwrap();
    /// assert!(g.position().board().is_occupied(square::E4));
    ///
    /// // Illegal move → None.
    /// let illegal = Uci{orig: square::E2, dest: square::E5, promotion: None};
    /// assert!(Game::new().mve(&illegal).is_none());
    /// ```
    pub fn mve(self, uci: &Uci) -> Option<Self> {
        self.position
            .mve(uci.orig, uci.dest, uci.promotion)
            .map(|position| {
                let outcome = eval(&position);
                Self { position, outcome }
            })
    }

    /// Returns `true` if it is white's turn to move.
    pub fn is_white_turn(&self) -> bool {
        self.position.color() == Color::White
    }

    /// Returns the terminal outcome of the game, if any has been reached.
    pub fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    /// Returns the current position.
    pub fn position(&self) -> &Position {
        &self.position
    }
}

/// Computes the terminal outcome of `position`, or `None` if the game is
/// still ongoing.
///
/// Draws are checked first (insufficient material, threefold repetition,
/// fifty-move rule), then absence of legal moves (checkmate vs. stalemate).
fn eval(position: &Position) -> Option<Outcome> {
    let has_moves = position.has_moves();

    if is_insufficient_material(position.board()) {
        Some(Outcome::Draw(DrawReason::InsufficientMaterial))
    } else if position.history().is_threefold_repetition() {
        Some(Outcome::Draw(DrawReason::ThreeFoldRepetition))
    } else if position.history().half_moves() >= 50 {
        Some(Outcome::Draw(DrawReason::FiftyMoveRule))
    } else if !has_moves && position.is_check() {
        let winner = position.color().opponent();
        Some(Outcome::Win(winner))
    } else if !has_moves {
        Some(Outcome::Draw(DrawReason::Stalemate))
    } else {
        None
    }
}

/// Returns `true` if `board` lacks the material for either side to deliver
/// checkmate.
///
/// Insufficient combinations: lone kings, K vs K+minor, K vs K+two knights,
/// or any number of bishops as long as they all share one color complex.
fn is_insufficient_material(board: &Board) -> bool {
    if board.pawns().is_non_empty() || board.rooks().is_non_empty() || board.queens().is_non_empty()
    {
        return false;
    }

    let minors = (board.knights() | board.bishops()).count();
    if minors <= 1 {
        return true;
    }
    if board.bishops().is_empty() && board.knights().count() == 2 {
        return true;
    }

    if board.knights().is_empty() {
        let bishops = board.bishops();
        return (bishops & Bitboard::LIGHT).is_empty() || (bishops & Bitboard::DARK).is_empty();
    }

    false
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.position.board())?;
        match self.outcome {
            Some(outcome) => write!(f, "{}", outcome),
            None => write!(
                f,
                "{:?} to move (move {})",
                self.position.ply().turn(),
                self.position.ply().full_move_number()
            ),
        }
    }
}
