/// Build ordered string pairs. Keys may repeat; expressions are evaluated once.
/// This is syntax reuse, not a substitute for domain validation.
#[macro_export]
macro_rules! string_pairs {
    ($($key:expr => $value:expr),* $(,)?) => {
        vec![$(($key.to_string(), $value.to_string())),*]
    };
}
