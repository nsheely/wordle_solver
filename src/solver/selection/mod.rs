//! Composite word selection strategies
//!
//! This module contains selection functions that combine multiple metrics
//! (entropy, minimax, expected values) to choose optimal guesses.
//!
//! Pure algorithms live in their respective modules:
//! - `entropy::selector` - Pure entropy maximization
//! - `minimax::selector` - Pure minimax optimization
//!
//! This module provides composite strategies used by `AdaptiveStrategy`.

pub mod adaptive;
pub mod hybrid;

pub use adaptive::{select_minimax_first, select_with_candidate_preference};
pub use hybrid::{select_with_expected_tiebreaker, select_with_hybrid_scoring};
