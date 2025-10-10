//! Terminal output formatting
//!
//! Display utilities for CLI results and pretty-printing.

pub mod display;
pub mod formatters;

pub use display::{print_analysis_result, print_benchmark_result, print_solve_result};
