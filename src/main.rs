use std::{
    error::Error,
    io::{self, Write},
};

use ruchess::{Game, Move};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();

    loop {
        println!("{:?}", game);
        let input = get_input("Move (e.g. e2e4): ");
        let mv: Move = match input.parse() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
        if let Err(e) = game.make_move(mv) {
            eprintln!("Invalid move: {e}");
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
