//! # ruchess
//!
//! A chess library built around [`Bitboard`](crate::bitboard::Bitboard)s and a
//! persistent, functional API. Every state-changing operation — placing a
//! piece, playing a move, updating castling rights — returns a new value
//! instead of mutating in place.
//!
//! ## At a glance
//!
//! ```
//! use ruchess::game::Game;
//! use ruchess::square;
//! use ruchess::uci::Uci;
//!
//! // Play 1.e4 from the standard starting position.
//! let game = Game::new();
//! let after_e4 = game.mve(&Uci { orig: square::E2, dest: square::E4, promotion: None }).unwrap();
//! assert!(after_e4.position().board().is_occupied(square::E4));
//! ```
//!
//! ## Module map
//!
//! - [`bitboard`] — 64-bit board representation and bitwise operations.
//! - [`square`], [`rank`], [`file`] — coordinate primitives.
//! - [`color`], [`role`], [`piece`], [`side`] — piece and side identifiers.
//! - [`board`] — piece placement, attack detection, and queries.
//! - [`attacks`], [`magic`] — precomputed attack tables and magic bitboards.
//! - [`mve`], [`uci`] — move representation and UCI parsing.
//! - [`castles`], [`unmoved_rooks`] — castling rights and rook tracking.
//! - [`halfmoveclock`], [`ply`] — clocks for the fifty-move rule and side to move.
//! - [`hash`] — Zobrist hashing and repetition trails.
//! - [`history`] — per-position history (last move, castles, clock, hashes).
//! - [`position`] — a complete game state with legal-move generation.
//! - [`outcome`] — terminal results (win, draw, draw reason).
//! - [`game`] — high-level wrapper that tracks turns and outcomes.
//! - [`fen`] — fen parser.

pub mod attacks;
pub mod bitboard;
pub mod board;
pub mod castles;
pub mod color;
pub mod fen;
pub mod file;
pub mod game;
pub mod halfmoveclock;
pub mod hash;
pub mod history;
pub mod magic;
pub mod mve;
pub mod outcome;
pub mod piece;
pub mod ply;
pub mod position;
pub mod rank;
pub mod role;
pub mod side;
pub mod square;
pub mod uci;
pub mod unmoved_rooks;
