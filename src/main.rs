use std::{error::Error, io};

use ruchess::Square;

fn main() -> Result<(), Box<dyn Error>> {
    let mut board = ruchess::Board::new();

    loop {
        println!("{}", board);
        let from: Square = get_input("From: ").parse()?;
        let to: Square = get_input("To: ").parse()?;
        board = board.move_(&from, &to);
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
