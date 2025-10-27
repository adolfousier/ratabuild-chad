// Utility modules

pub mod logger;
pub mod helpers;

// Re-export commonly used functions
pub use helpers::{detect_language_for_path, calculate_dir_size};
