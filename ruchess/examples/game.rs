use ruchess::{game::Game, uci::Uci};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();

    while game.outcome().is_none() {
        println!("\n\n{}", game);
        let uci = get_input("format: <from><to><optional promotion role>\nexample: e2e4, e7e8q");
        if let Some(g) = game.clone().mve(uci.orig, uci.dest) {
            game = g;
        }
    }

    Ok(())
}

fn get_input(help: &str) -> Uci {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");

        if let Ok(uci) = input.trim().parse() {
            return uci;
        }
        println!("{help}");
    }
}
