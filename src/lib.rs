//! Wordle Solver
//!
//! A Wordle solver using information theory and game theory, achieving 99.7-99.8% optimal performance.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wordle_solver::core::{Word, Pattern};
//!
//! // Create words
//! let guess = Word::new("crane").unwrap();
//! let answer = Word::new("slate").unwrap();
//!
//! // Calculate pattern
//! let pattern = Pattern::calculate(&guess, &answer);
//! println!("Pattern value: {}", pattern.value());
//! ```

// Core domain types
pub mod core;

// Solving algorithms
pub mod solver;

// Word lists
pub mod wordlists;

// Command implementations
pub mod commands;

// Terminal output formatting
pub mod output;

// Interactive TUI interface
pub mod interactive;
