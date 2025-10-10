//! Core domain types for Wordle
//!
//! This module contains the fundamental domain types with zero external dependencies.
//! All types here are pure, testable, and have clear mathematical properties.

mod pattern;
mod word;

pub use pattern::Pattern;
pub use word::Word;
