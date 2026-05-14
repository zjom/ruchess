//! # Castling Side
//!
//! Identifies which flank a castling move targets. Used together with
//! [`Color`](crate::color::Color) to address a specific castling right
//! (e.g. White king-side) or the rook involved.

/// The side of the board a castling move targets.
///
/// `King` corresponds to king-side castling (O-O); `Queen` corresponds to
/// queen-side castling (O-O-O).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// King-side (short) castle.
    King,
    /// Queen-side (long) castle.
    Queen,
}
