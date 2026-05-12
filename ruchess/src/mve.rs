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
