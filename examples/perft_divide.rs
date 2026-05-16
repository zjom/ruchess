//! perft divide — prints the number of leaf nodes per root move at a given depth.
//!
//! Usage:
//!     cargo run --release --example perft_divide -- "<FEN>" <depth>

use ruchess::{fen, position::Position};

fn perft(pos: &mut Position, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = pos.valid_moves();
    if depth == 1 {
        return moves.len() as u64;
    }
    let mut total: u64 = 0;
    for m in moves {
        let undo = pos.make(&m);
        total += perft(pos, depth - 1);
        pos.unmake(undo);
    }
    total
}

fn fmt_move(m: &ruchess::mve::Move) -> String {
    let promo = match m.promo_role() {
        Some(p) => format!("{}", p.as_ascii()),
        None => String::new(),
    };
    format!("{}{}{}", m.orig(), m.dest(), promo)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: perft_divide <FEN> <depth>");
        std::process::exit(2);
    }
    let fen_str = &args[0];
    let depth: u32 = args[1].parse().expect("depth must be a number");

    let pos = fen::parse(fen_str)
        .expect("FEN should parse")
        .without_repetition();

    let moves = pos.valid_moves();
    let mut rows: Vec<(String, u64)> = moves
        .into_iter()
        .map(|m| {
            let mut working = pos.clone();
            let undo = working.make(&m);
            let count = if depth == 0 {
                1
            } else {
                perft(&mut working, depth - 1)
            };
            working.unmake(undo);
            (fmt_move(&m), count)
        })
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total: u64 = 0;
    for (mv, n) in &rows {
        println!("{mv}: {n}");
        total += n;
    }
    println!("\nmoves: {}", rows.len());
    println!("total: {total}");
}
