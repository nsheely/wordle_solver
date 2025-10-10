//! Command implementations

pub mod analyze;
pub mod benchmark;
pub mod simple;
pub mod solve;
pub mod test_all;

pub use analyze::{AnalysisResult, analyze_word};
pub use benchmark::{BenchmarkResult, run_benchmark};
pub use simple::run_simple;
pub use solve::{SolveConfig, SolveResult, solve_word};
pub use test_all::{TestAllStatistics, print_test_all_statistics, run_test_all};
