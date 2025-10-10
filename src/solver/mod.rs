//! Wordle solving algorithms
//!
//! This module contains different solving strategies for Wordle.

pub mod adaptive;
mod engine;
pub mod entropy;
pub mod minimax;
pub mod selection;
pub mod strategy;

pub use adaptive::{AdaptiveStrategy, AdaptiveTier};
pub use engine::Solver;
pub use strategy::{EntropyStrategy, HybridStrategy, MinimaxStrategy, Strategy, StrategyType};
