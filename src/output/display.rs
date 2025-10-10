//! Display functions for command results

use super::formatters::{entropy_bar, pattern_to_emoji};
use crate::commands::{AnalysisResult, BenchmarkResult, SolveResult};
use colored::Colorize;

/// Print the result of solving a word
pub fn print_solve_result(result: &SolveResult, verbose: bool) {
    println!("\n{}", "─".repeat(60).cyan());
    println!(
        "Solving: {}",
        result.target.to_uppercase().bright_yellow().bold()
    );
    println!("{}", "─".repeat(60).cyan());

    for (i, step) in result.guesses.iter().enumerate() {
        let turn = i + 1;
        println!(
            "\nTurn {}: {} {}",
            turn,
            step.word.to_uppercase(),
            pattern_to_emoji(step.pattern)
        );

        if verbose {
            println!(
                "  Candidates: {} → {}",
                step.candidates_before, step.candidates_after
            );

            if let Some(entropy) = step.entropy {
                println!("  Entropy:    {entropy:.3} bits");
                if let Some(expected) = step.expected_remaining {
                    println!("  Expected:   {expected:.1} candidates");
                }

                // Calculate information gained (reduction in uncertainty)
                if step.candidates_after > 0 {
                    let actual_reduction =
                        (step.candidates_before as f64 / step.candidates_after as f64).log2();
                    println!(
                        "  Info gained: {:.3} bits ({:.1}x reduction)",
                        actual_reduction,
                        step.candidates_before as f64 / step.candidates_after as f64
                    );
                }
            }
        }
    }

    println!();
    if result.success {
        println!(
            "{}",
            format!("✅ Solved in {} guesses!", result.guesses.len())
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            format!("❌ Failed to solve in {} guesses", result.guesses.len())
                .red()
                .bold()
        );
    }
}

/// Print the result of word analysis
pub fn print_analysis_result(result: &AnalysisResult) {
    println!("\n{}", "═".repeat(60).cyan());
    println!(
        " {} {} ",
        "ENTROPY ANALYSIS:".bright_cyan().bold(),
        result.word.to_uppercase().bright_yellow().bold()
    );
    println!("{}", "═".repeat(60).cyan());

    let bar = entropy_bar(result.entropy, 30);

    println!("\n📊 Against {} possible answers:", result.total_candidates);
    println!(
        "   Entropy:     [{}] {}",
        bar.green(),
        format!("{:.3} bits", result.entropy).bright_yellow()
    );
    println!(
        "   Info gain:   {:.1}x reduction",
        result.expected_reduction
    );
    println!(
        "   Expected:    {:.1} candidates remain",
        result.expected_remaining
    );
}

/// Print the result of a benchmark
pub fn print_benchmark_result(result: &BenchmarkResult) {
    println!("\n{}", "═".repeat(60).cyan());
    println!(" {} ", "BENCHMARK RESULTS".bright_cyan().bold());
    println!("{}", "═".repeat(60).cyan());

    println!("\n📊 {}", "Performance:".bright_cyan().bold());
    println!("   Words tested:     {}", result.total_words);
    println!(
        "   Average guesses:  {}",
        format!("{:.2}", result.average_guesses)
            .bright_yellow()
            .bold()
    );
    println!(
        "   Best case:        {}",
        format!("{}", result.min_guesses).green()
    );
    println!(
        "   Worst case:       {}",
        format!("{}", result.max_guesses).yellow()
    );
    println!("   Time taken:       {:.2}s", result.duration.as_secs_f64());
    println!("   Words/second:     {:.1}", result.words_per_second);

    println!("\n📈 {}", "Distribution:".bright_cyan().bold());
    for guess_count in 1..=6 {
        if let Some(&count) = result.distribution.get(&guess_count) {
            let pct = (count as f64 / result.total_words as f64) * 100.0;
            let bar_width = (pct / 2.5) as usize;
            let bar = format!(
                "{}{}",
                "█".repeat(bar_width).green(),
                "░"
                    .repeat(40_usize.saturating_sub(bar_width))
                    .bright_black()
            );
            println!("   {guess_count}: {bar} {count:4} ({pct:5.1}%)");
        }
    }
}
