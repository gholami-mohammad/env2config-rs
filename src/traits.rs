/// A trait for creating an instance of a type from environment variables.
///
/// Types that implement this trait should provide a way to initialize themselves
/// using values from the environment. This is useful for configuration purposes,
/// where settings can be provided through environment variables.
///
/// # Examples
///
/// ```
/// use env2config::FromEnv;
/// use std::env::set_var;
///
/// #[derive(FromEnv)]
/// struct Config {
///     #[env("DATABASE_URL")]
///     database_url: String,
///
///     #[env("HOST", "127.0.0.1")] // HOST is env variable name and 127.0.0.1 is default value if HOST is not provided
///     host: String,
///
///     #[env("PORT")]
///     port: i32,
///     
///     // loading comma separated values into vectors of these types:
///     // String, bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
///     #[env("IDS", "1,2,3")]
///     ids: Vec<i32>,
///
///     #[env("SYMBOLS")]
///     symbols: Vec<String>,
/// }
///
/// fn main() {
///     unsafe {
///         set_var("PORT", "5544");
///         set_var("SYMBOLS", "A,B,C");
///     }
///     let cfg = Config::from_env();
///     assert_eq!(cfg.host, "127.0.0.1");
///     assert_eq!(cfg.port, 5544);
///     assert_eq!(cfg.ids.len(), 3);
///     assert_eq!(cfg.ids[0], 1);
///     assert_eq!(cfg.symbols.len(), 3);
///     assert_eq!(cfg.symbols[0], "A");
///     assert_eq!(cfg.symbols[1], "B");
///     assert_eq!(cfg.symbols[2], "C");
/// }
/// ```
pub trait FromEnv {
    fn from_env() -> Self;
}
