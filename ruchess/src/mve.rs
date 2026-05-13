use crate::{
    board::Board,
    color::Color,
    piece::Piece,
    role::{PromotableRole, Role},
    side::Side,
    square::Square,
};

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub piece: Piece,
    pub orig: Square,
    pub dest: Square,
    pub capture: Option<Square>,
    pub promotion: Option<PromotableRole>,
    pub castle: Option<Side>,
    pub enpassant: Option<()>,
    pub after: Board,
    pub previous: Board,
}
