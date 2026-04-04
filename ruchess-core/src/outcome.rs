use crate::Color;

pub enum Outcome {
    Ongoing,
    Win(Color),
    Draw,
}
