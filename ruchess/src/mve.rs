use std::vec;

use crate::{
    attacks::ATTACKS,
    board::Board,
    color::Color,
    piece::Piece,
    role::{PromotableRole, Role},
    square::Square,
};

pub struct Move {
    pub piece: Piece,
    pub orig: Square,
    pub dest: Square,
    pub capture: Option<Square>,
    pub promotion: Option<PromotableRole>,
    pub castle: Option<Castle>,
    pub enpassant: Option<()>,
    pub after: Board,
}

impl Move {
    /// Builds a non-promotion [`Move`] from `orig` to `dest`.
    ///
    /// When `is_capture` is `true`, any piece on `dest` is taken; otherwise the
    /// move requires `dest` to be empty. Returns `None` if the underlying board
    /// transition is illegal (e.g. moving from an empty square, or pushing onto
    /// an occupied square when `is_capture` is `false`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// # use ruchess::mve::Move;
    /// # use ruchess::role::Role;
    /// # use ruchess::square::{A2, A3};
    /// let m = Move::normal(Board::new(), A2, A3, false, Color::White, Role::Pawn)
    ///     .expect("a2-a3 is legal from the starting position");
    /// assert_eq!(m.orig, A2);
    /// assert_eq!(m.dest, A3);
    /// assert!(m.capture.is_none());
    /// ```
    ///
    /// Pushing onto an occupied square without `is_capture` fails:
    ///
    /// ```
    /// # use ruchess::board::Board;
    /// # use ruchess::color::Color;
    /// # use ruchess::mve::Move;
    /// # use ruchess::role::Role;
    /// # use ruchess::square::{A1, A2};
    /// assert!(Move::normal(Board::new(), A1, A2, false, Color::White, Role::Rook).is_none());
    /// ```
    pub fn normal(
        board: Board,
        orig: Square,
        dest: Square,
        is_capture: bool,
        color: Color,
        role: Role,
    ) -> Option<Move> {
        let taken = is_capture.then_some(dest);
        let after = if is_capture {
            board.capture(orig, dest)
        } else {
            board.mve(orig, dest)
        };

        after.map(|after| Move {
            piece: Piece { role, color },
            orig,
            dest,
            capture: taken,
            promotion: None,
            castle: None,
            enpassant: None,
            after,
        })
    }
}

pub enum Castle {
    KingSide,
    QueenSide,
}

/// Generates all pseudo-legal pawn moves for `color` from `board`.
///
/// Includes captures (diagonal onto opponent pieces), single pushes (one square
/// forward onto an empty square), and double pushes (two squares forward from
/// the starting rank when both intermediate and target squares are empty).
///
/// # Examples
///
/// From the standard starting position, each side has eight single pushes and
/// eight double pushes:
///
/// ```
/// # use ruchess::board::Board;
/// # use ruchess::color::Color;
/// # use ruchess::mve::pawn_moves;
/// assert_eq!(pawn_moves(Board::new(), Color::White).len(), 16);
/// assert_eq!(pawn_moves(Board::new(), Color::Black).len(), 16);
/// ```
pub fn pawn_moves(board: Board, color: Color) -> Vec<Move> {
    let pawns = board.bypiece(Piece {
        role: Role::Pawn,
        color,
    });
    let captures = pawns
        .flat_map(|from| {
            (ATTACKS.pawn_attacks(color, from) & board.bycolor(color.opponent()))
                .into_iter()
                .map(move |to| (from, to))
        })
        .flat_map(|(from, to)| gen_pawn_moves(board, from, to, color, true));

    let singles = !board.occupied()
        & (match color {
            Color::White => (board.bycolor(color) & pawns) << 8,
            Color::Black => (board.bycolor(color) & pawns) >> 8,
        });

    let single_moves = singles.flat_map(|to| {
        let from = Square(match color {
            Color::White => to.0 - 8,
            Color::Black => to.0 + 8,
        });
        gen_pawn_moves(board, from, to, color, false)
    });

    let doubles = !board.occupied()
        & (match color {
            Color::White => singles << 8,
            Color::Black => singles >> 8,
        })
        & color.fourth_rank();
    let double_moves = doubles.flat_map(|to| {
        let from = Square(match color {
            Color::White => to.0 - 16,
            Color::Black => to.0 + 16,
        });
        gen_pawn_moves(board, from, to, color, false)
    });

    captures.chain(single_moves).chain(double_moves).collect()
}

fn gen_pawn_moves(
    board: Board,
    from: Square,
    to: Square,
    color: Color,
    is_capture: bool,
) -> Vec<Move> {
    if from.rank() == color.seventh_rank() {
        PromotableRole::ROLES
            .into_iter()
            .filter_map(|r| {
                if is_capture {
                    board.capture(from, to).map(|after| Move {
                        piece: Piece {
                            role: Role::Pawn,
                            color,
                        },
                        orig: from,
                        dest: to,
                        capture: Some(to),
                        promotion: Some(r),
                        castle: None,
                        enpassant: None,
                        after,
                    })
                } else {
                    board.mve(from, to).map(|after| Move {
                        piece: Piece {
                            role: Role::Pawn,
                            color,
                        },
                        orig: from,
                        dest: to,
                        capture: None,
                        promotion: Some(r),
                        castle: None,
                        enpassant: None,
                        after,
                    })
                }
            })
            .collect()
    } else {
        Move::normal(board, from, to, is_capture, color, Role::Pawn)
            .map(|m| vec![m])
            .unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::{A1, A2, A3, A4, B7, D5, E4, E5};

    fn board_with(pieces: &[(Square, Piece)]) -> Board {
        pieces
            .iter()
            .fold(Board::EMPTY, |b, (sq, p)| b.set(*sq, *p))
    }

    // ── Move::normal ─────────────────────────────────────────────────────────

    #[test]
    fn normal_move_onto_empty_square_succeeds() {
        let m = Move::normal(Board::new(), A2, A3, false, Color::White, Role::Pawn)
            .expect("a2-a3 should be legal from the starting position");

        assert_eq!(m.orig, A2);
        assert_eq!(m.dest, A3);
        assert!(m.capture.is_none());
        assert!(m.promotion.is_none());
        assert!(m.castle.is_none());
        assert!(m.enpassant.is_none());

        // The piece left its origin and now lives on the destination.
        assert!(!m.after.is_occupied(A2));
        let landed = m.after.piece_at(A3).expect("pawn should be on a3");
        assert_eq!(landed.role, Role::Pawn);
        assert_eq!(landed.color, Color::White);
    }

    #[test]
    fn normal_non_capture_onto_occupied_square_returns_none() {
        // a1 holds the white rook; trying to slide it onto its own pawn on a2
        // without `is_capture` must fail.
        assert!(Move::normal(Board::new(), A1, A2, false, Color::White, Role::Rook).is_none());
    }

    #[test]
    fn normal_from_empty_origin_returns_none() {
        let b = Board::EMPTY;
        assert!(Move::normal(b, A1, A2, false, Color::White, Role::Rook).is_none());
    }

    #[test]
    fn normal_capture_records_target_square_and_replaces_piece() {
        let b = board_with(&[
            (
                E4,
                Piece {
                    role: Role::Knight,
                    color: Color::White,
                },
            ),
            (
                D5,
                Piece {
                    role: Role::Pawn,
                    color: Color::Black,
                },
            ),
        ]);

        let m = Move::normal(b, E4, D5, true, Color::White, Role::Knight)
            .expect("knight should be able to capture on d5");

        assert_eq!(m.capture, Some(D5));
        assert!(!m.after.is_occupied(E4));
        let landed = m.after.piece_at(D5).expect("knight should be on d5");
        assert_eq!(landed.role, Role::Knight);
        assert_eq!(landed.color, Color::White);
    }

    // ── pawn_moves ───────────────────────────────────────────────────────────

    #[test]
    fn pawn_moves_from_starting_position_white() {
        let moves = pawn_moves(Board::new(), Color::White);
        // 8 single pushes + 8 double pushes, no captures available.
        assert_eq!(moves.len(), 16);
        assert!(moves.iter().all(|m| m.capture.is_none()));
        assert!(moves.iter().all(|m| m.promotion.is_none()));
    }

    #[test]
    fn pawn_moves_from_starting_position_black() {
        let moves = pawn_moves(Board::new(), Color::Black);
        assert_eq!(moves.len(), 16);
        assert!(moves.iter().all(|m| m.capture.is_none()));
    }

    #[test]
    fn pawn_double_push_blocked_when_intermediate_square_occupied() {
        // A friendly piece on a3 blocks both a2-a3 and the a2-a4 double push.
        let b = Board::new().set(
            A3,
            Piece {
                role: Role::Knight,
                color: Color::White,
            },
        );

        let from_a2: Vec<_> = pawn_moves(b, Color::White)
            .into_iter()
            .filter(|m| m.orig == A2)
            .collect();
        assert!(from_a2.is_empty());
    }

    #[test]
    fn pawn_double_push_blocked_when_target_square_occupied() {
        // An enemy piece on a4 leaves the single push open but cancels the double.
        let b = Board::new().set(
            A4,
            Piece {
                role: Role::Pawn,
                color: Color::Black,
            },
        );

        let from_a2: Vec<_> = pawn_moves(b, Color::White)
            .into_iter()
            .filter(|m| m.orig == A2)
            .collect();
        assert_eq!(from_a2.len(), 1);
        assert_eq!(from_a2[0].dest, A3);
        assert!(from_a2[0].capture.is_none());
    }

    #[test]
    fn pawn_captures_diagonally_onto_enemy_piece() {
        // Lone white pawn on e4, lone black pawn on d5 — d5 must be flagged as a capture.
        let b = board_with(&[
            (
                E4,
                Piece {
                    role: Role::Pawn,
                    color: Color::White,
                },
            ),
            (
                D5,
                Piece {
                    role: Role::Pawn,
                    color: Color::Black,
                },
            ),
        ]);

        let captures: Vec<_> = pawn_moves(b, Color::White)
            .into_iter()
            .filter(|m| m.capture.is_some())
            .collect();

        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].orig, E4);
        assert_eq!(captures[0].dest, D5);
        assert_eq!(captures[0].capture, Some(D5));
        // The pawn actually moved on the resulting board.
        assert!(!captures[0].after.is_occupied(E4));
        let landed = captures[0].after.piece_at(D5).unwrap();
        assert_eq!(landed.role, Role::Pawn);
        assert_eq!(landed.color, Color::White);
    }

    #[test]
    fn pawn_does_not_capture_own_color() {
        // Two adjacent white pawns: e4 and d5. e4 must not "capture" d5.
        let b = board_with(&[
            (
                E4,
                Piece {
                    role: Role::Pawn,
                    color: Color::White,
                },
            ),
            (
                D5,
                Piece {
                    role: Role::Pawn,
                    color: Color::White,
                },
            ),
        ]);

        let captures: Vec<_> = pawn_moves(b, Color::White)
            .into_iter()
            .filter(|m| m.capture.is_some())
            .collect();
        assert!(captures.is_empty());
    }

    #[test]
    fn lone_pawn_at_e4_has_only_a_single_push() {
        let b = board_with(&[(
            E4,
            Piece {
                role: Role::Pawn,
                color: Color::White,
            },
        )]);

        let moves = pawn_moves(b, Color::White);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].orig, E4);
        assert_eq!(moves[0].dest, E5);
        assert!(moves[0].capture.is_none());
    }

    #[test]
    fn black_pawn_attacks_two_white_targets_with_two_captures() {
        // Black pawn on b7 with two white pieces diagonally in front: a6 and c6.
        // Use a7 / c7 squares — but b7's black-pawn attacks land on a6 and c6.
        use crate::square::{A6, C6};
        let b = board_with(&[
            (
                B7,
                Piece {
                    role: Role::Pawn,
                    color: Color::Black,
                },
            ),
            (
                A6,
                Piece {
                    role: Role::Pawn,
                    color: Color::White,
                },
            ),
            (
                C6,
                Piece {
                    role: Role::Pawn,
                    color: Color::White,
                },
            ),
        ]);

        let captures: Vec<_> = pawn_moves(b, Color::Black)
            .into_iter()
            .filter(|m| m.capture.is_some())
            .collect();
        assert_eq!(captures.len(), 2);
        let dests: Vec<_> = captures.iter().map(|m| m.dest).collect();
        assert!(dests.contains(&A6));
        assert!(dests.contains(&C6));
    }
}
