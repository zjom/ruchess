# Ruchess

> [!IMPORTANT]
> THIS PROJECT IS UNDER ACTIVE DEVELOPMENT.
> THERE ARE CORE FUNCTIONALITIES THAT HAVE NOT BEEN IMPLEMENTED YET
> DO NOT USE.

Chess library, tui and engine written in Rust.

Contains:

- core chess library in [ruchess-core](ruchess-core/)
- tui in [./ruchess-bin](ruchess-bin/)
- engine in [./ruchess-engine](./ruchess-engine) !NOT IMPLEMENTED

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
ruchess-core = { path = "<path to repo root>/ruchess-core" }
```

```rust
use ruchess_core::{Game, Move};

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
cargo run <path to repo root>
```

## Roadmap

**CORE**

- [ ] enforce checks
- [ ] end of game determination
- [ ] optimise attack calculations using precomputed magic numbers
- [ ] serialize/deserialize game/move encoding formats
  - [ ] [ACN](<https://en.wikipedia.org/wiki/Algebraic_notation_(chess)>)
  - [ ] [FEN](https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation)
  - [ ] [PGN](https://en.wikipedia.org/wiki/Portable_Game_Notation)

**Engine**
