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
env2config = "1.0.0"
```

```rust
use env2config::FromEnv;

#[derive(FromEnv)]
struct Config {
    #[env("DATABASE_URL")]
    database_url: String,

    #[env("HOST", "127.0.0.1")] // HOST is env variable name and 127.0.0.1 is default value if HOST is not provided
    host: String,

    #[env("PORT", "8080")]
    port: u16,

    #[env("DEBUG", "false")]
    debug: bool,
}

fn main() {
    let cfg = Config::from_env();
    println!("Database URL: {}", cfg.database_url);
    println!("Host: {}", cfg.host);
    println!("Port: {}", cfg.port);
    println!("Debug: {}", cfg.debug);
}
```

License

This project is licensed under the MIT License - see the LICENSE file for details.
