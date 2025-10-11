//! Minimax guess selection
//!
//! Minimizes worst-case remaining candidates.

mod calculator;
mod selector;

pub use calculator::calculate_max_remaining;
pub use selector::select_best_guess;
