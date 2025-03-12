# env2config

`env2config` is a Rust crate that allows you to load environment variables into structs. This is useful for configuration purposes, where settings can be provided through environment variables.

## Features

-   Load environment variables into structs
-   Support for default values
-   Support for various data types including `String`, `bool`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, and `Vec<T>`

## Usage

Add `env2config` to your `Cargo.toml`:

```toml
[dependencies]
env2config = "1.0.2"
```

```rust
use env2config::FromEnv;
use std::env::set_var;

#[derive(FromEnv)]
struct Config {
    #[env("DATABASE_URL")]
    database_url: String,

    #[env("HOST", "127.0.0.1")] // HOST is env variable name and 127.0.0.1 is default value if HOST is not provided
    host: String,

    #[env("PORT")]
    port: i32,

    // loading comma separated values into vectors of these types:
    // String, bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
    #[env("IDS", "1,2,3")]
    ids: Vec<i32>,

    #[env("SYMBOLS")]
    symbols: Vec<String>,
}

fn main() {
    unsafe {
        set_var("PORT", "5544");
        set_var("SYMBOLS", "A,B,C");
    }
    let cfg = Config::from_env();
    assert_eq!(cfg.host, "127.0.0.1");
    assert_eq!(cfg.port, 5544);
    assert_eq!(cfg.ids.len(), 3);
    assert_eq!(cfg.ids[0], 1);
    assert_eq!(cfg.symbols.len(), 3);
    assert_eq!(cfg.symbols[0], "A");
    assert_eq!(cfg.symbols[1], "B");
    assert_eq!(cfg.symbols[2], "C");
}
```

License

This project is licensed under the MIT License - see the LICENSE file for details.
