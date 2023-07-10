use std::fmt::Display;

/// the rgb css function
pub fn rgb(r: impl Display, g: impl Display, b: impl Display) -> String {
    format!("rgb({r}, {g}, {b})")
}
