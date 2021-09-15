macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}
