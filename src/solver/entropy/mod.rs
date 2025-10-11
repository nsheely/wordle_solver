//! Entropy-based guess selection
//!
//! Calculates entropy for Wordle pattern distributions.

mod calculator;
mod selector;

pub use calculator::{GuessMetrics, calculate_entropy, calculate_metrics, shannon_entropy};
pub use selector::select_best_guess;
