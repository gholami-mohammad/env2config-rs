/// A trait for creating an instance of a type from environment variables.
///
/// Types that implement this trait should provide a way to initialize themselves
/// using values from the environment. This is useful for configuration purposes,
/// where settings can be provided through environment variables.
///
/// # Examples
///
/// ```
/// use envconfig::FromEnv;
/// use envconfig_derive::FromEnv;
///
/// #[derive(FromEnv)]
/// struct Config {
///     #[env("DATABASE_URL")]
///     database_url: String,
/// }
///
/// fn main() {
///     let cfg = Config::from_env();
///     println!("{}", cfg.database_url);
/// }
/// ```
pub trait FromEnv {
    fn from_env() -> Self;
}
