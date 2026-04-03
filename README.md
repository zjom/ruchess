# Ruchess

> [!IMPORTANT]
> THIS PROJECT IS A WIP.
> THERE ARE CORE FUNCTIONALITIES THAT HAVE NOT BEEN IMPLEMENTED YET
> DO NOT USE.

Chess engine written in Rust.

Contains:
- core chess engine in `./ruchess/`
- tui in `./ruchess-bin/`

## Usage
Ruchess is not yet published on crates.io.

First clone this repo.

```sh
git clone github.com/zjom/ruchess --head
```

### As library

Update your `cargo.toml` file with a `dependencies` section pointing at the path of where you cloned this project.

Example: 
```toml
[package]
name = "urpackage"
version = "0.1.0"
edition = "2024"

[dependencies]
ruchess = { path = "<path to repo root>/ruchess" }
```

```rust
use ruchess::{Game, Move};

fn main()->{
    let mut game = Game::new();
    println!("{}",game);
    let m:Move = "e2e4".parse().expect("failed to parse move");
    game.make_move(m).expect("invalid move");
    println!("{}",game);
}
```


### As a TUI

```sh
cargo run
```

