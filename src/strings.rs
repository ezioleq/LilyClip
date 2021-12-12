//! Some constraints used in the program

/// Application title
pub const TITLE: &str = "LilyClip";
/// Header bar's subtitle
pub const SUBTITLE: &str = "Your frequently used phrases";
/// Author of the application (used in About dialog)
pub const AUTHOR: &str = "Ezioleq";
/// Application version string
///
/// Uses environment variable CARGO_PKG_VERSION which is passed by cargo
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
