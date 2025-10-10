//! Entropy-based Wordle solver
//!
//! Implements Shannon entropy calculation for Wordle pattern distributions.
//! This is the foundation of information-theoretic solving.

mod calculator;
mod selector;

pub use calculator::{GuessMetrics, calculate_entropy, calculate_metrics, shannon_entropy};
pub use selector::select_best_guess;
