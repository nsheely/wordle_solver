//! Minimax-based Wordle solver
//!
//! Implements worst-case minimization strategy for Wordle solving.

mod calculator;
mod selector;

pub use calculator::calculate_max_remaining;
pub use selector::select_best_guess;
