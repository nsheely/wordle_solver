//! Shannon entropy calculation for Wordle patterns
//!
//! Given a guess and set of candidates, computes the expected information gain.

use crate::core::{Pattern, Word};
use rustc_hash::FxHashMap;

/// Comprehensive metrics for evaluating a guess
#[derive(Debug, Clone, Copy)]
pub struct GuessMetrics {
    /// Shannon entropy (expected information gain in bits)
    pub entropy: f64,
    /// Expected number of remaining candidates after this guess
    pub expected_remaining: f64,
    /// Maximum partition size (worst-case remaining candidates)
    pub max_partition: usize,
}

/// Calculate Shannon entropy for a guess against candidates
///
/// Returns the expected information gain in bits.
///
/// # Formula
/// H(X) = -Σ p(x) * log₂(p(x))
///
/// where p(x) is the probability of observing pattern x.
///
/// # Examples
/// ```
/// use wordle_entropy::core::Word;
/// use wordle_entropy::solver::entropy::calculate_entropy;
///
/// let guess = Word::new("crane").unwrap();
/// let candidates = vec![
///     Word::new("slate").unwrap(),
///     Word::new("irate").unwrap(),
/// ];
/// let candidate_refs: Vec<&Word> = candidates.iter().collect();
///
/// let entropy = calculate_entropy(&guess, &candidate_refs);
/// assert!(entropy > 0.0 && entropy <= 1.0); // log2(2) = 1 bit max
/// ```
#[must_use]
pub fn calculate_entropy(guess: &Word, candidates: &[&Word]) -> f64 {
    if candidates.is_empty() {
        return 0.0;
    }

    // Group candidates by pattern
    let pattern_counts = group_by_pattern(guess, candidates);

    // Calculate Shannon entropy
    shannon_entropy(&pattern_counts)
}

/// Group candidates by the pattern they produce with the guess
fn group_by_pattern(guess: &Word, candidates: &[&Word]) -> FxHashMap<Pattern, usize> {
    let mut counts = FxHashMap::default();

    for &candidate in candidates {
        let pattern = Pattern::calculate(guess, candidate);
        *counts.entry(pattern).or_insert(0) += 1;
    }

    counts
}

/// Calculate Shannon entropy from pattern distribution
///
/// H = -Σ p * log₂(p)
///
/// # Properties
/// - Returns 0.0 for certain outcome (one pattern with p=1)
/// - Maximized for uniform distribution
/// - Always in range [0, log₂(n)] for n patterns
///
/// # Examples
/// ```
/// use wordle_entropy::solver::entropy::shannon_entropy;
/// use rustc_hash::FxHashMap;
/// use wordle_entropy::core::Pattern;
///
/// let mut uniform = FxHashMap::default();
/// uniform.insert(Pattern::new(0), 25);
/// uniform.insert(Pattern::new(1), 25);
/// uniform.insert(Pattern::new(2), 25);
/// uniform.insert(Pattern::new(3), 25);
///
/// let entropy = shannon_entropy(&uniform);
/// assert!((entropy - 2.0).abs() < 0.001); // log2(4) = 2 bits
/// ```
#[must_use]
pub fn shannon_entropy<S>(pattern_counts: &std::collections::HashMap<Pattern, usize, S>) -> f64
where
    S: std::hash::BuildHasher,
{
    let total = pattern_counts.values().sum::<usize>() as f64;

    if total == 0.0 {
        return 0.0;
    }

    pattern_counts
        .values()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / total;
            -p * p.log2()
        })
        .sum()
}

/// Calculate comprehensive metrics for a guess
///
/// Returns entropy, expected remaining candidates, and max partition size.
/// This enables sophisticated tiebreaking strategies.
pub fn calculate_metrics(guess: &Word, candidates: &[&Word]) -> GuessMetrics {
    if candidates.is_empty() {
        return GuessMetrics {
            entropy: 0.0,
            expected_remaining: 0.0,
            max_partition: 0,
        };
    }

    let mut pattern_groups: FxHashMap<Pattern, Vec<&Word>> = FxHashMap::default();

    // Group candidates by pattern
    for &candidate in candidates {
        let pattern = Pattern::calculate(guess, candidate);
        pattern_groups.entry(pattern).or_default().push(candidate);
    }

    let total = candidates.len() as f64;

    // Calculate entropy
    let entropy: f64 = pattern_groups
        .values()
        .map(|group| {
            let p = group.len() as f64 / total;
            -p * p.log2()
        })
        .sum();

    // Calculate expected remaining candidates
    let expected_remaining: f64 = pattern_groups
        .values()
        .map(|group| {
            let p = group.len() as f64 / total;
            p * group.len() as f64
        })
        .sum();

    // Find max partition size (minimax worst-case)
    let max_partition = pattern_groups
        .values()
        .map(std::vec::Vec::len)
        .max()
        .unwrap_or(0);

    GuessMetrics {
        entropy,
        expected_remaining,
        max_partition,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shannon_entropy_uniform_distribution() {
        // 4 patterns, each appears once = log2(4) = 2 bits
        let mut counts = FxHashMap::default();
        counts.insert(Pattern::new(0), 1);
        counts.insert(Pattern::new(1), 1);
        counts.insert(Pattern::new(2), 1);
        counts.insert(Pattern::new(3), 1);

        let entropy = shannon_entropy(&counts);
        assert!((entropy - 2.0).abs() < 0.001);
    }

    #[test]
    fn shannon_entropy_certain_outcome() {
        // Only one pattern = 0 bits (no uncertainty)
        let mut counts = FxHashMap::default();
        counts.insert(Pattern::new(0), 10);

        let entropy = shannon_entropy(&counts);
        assert!(entropy.abs() < 0.001);
    }

    #[test]
    fn shannon_entropy_skewed_distribution() {
        // Skewed distribution has less entropy than uniform
        let mut uniform = FxHashMap::default();
        uniform.insert(Pattern::new(0), 25);
        uniform.insert(Pattern::new(1), 25);
        uniform.insert(Pattern::new(2), 25);
        uniform.insert(Pattern::new(3), 25);

        let mut skewed = FxHashMap::default();
        skewed.insert(Pattern::new(0), 97);
        skewed.insert(Pattern::new(1), 1);
        skewed.insert(Pattern::new(2), 1);
        skewed.insert(Pattern::new(3), 1);

        assert!(shannon_entropy(&uniform) > shannon_entropy(&skewed));
    }

    #[test]
    fn shannon_entropy_bounds() {
        // Entropy is always non-negative and bounded by log2(n)
        let mut counts = FxHashMap::default();
        counts.insert(Pattern::new(0), 10);
        counts.insert(Pattern::new(1), 20);
        counts.insert(Pattern::new(2), 30);

        let entropy = shannon_entropy(&counts);
        assert!(entropy >= 0.0);
        assert!(entropy <= (counts.len() as f64).log2());
    }

    #[test]
    fn shannon_entropy_empty() {
        let counts: FxHashMap<Pattern, usize> = FxHashMap::default();
        let entropy = shannon_entropy(&counts);
        assert!((entropy - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn calculate_entropy_real_words() {
        let guess = Word::new("crane").unwrap();
        let candidates = vec![
            Word::new("slate").unwrap(),
            Word::new("irate").unwrap(),
            Word::new("trace").unwrap(),
            Word::new("raise").unwrap(),
        ];
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let entropy = calculate_entropy(&guess, &candidate_refs);

        // With 4 candidates and good diversity, expect 1.5-2.0 bits
        assert!(entropy > 1.0 && entropy <= 2.0);
    }

    #[test]
    fn calculate_entropy_all_same_pattern() {
        // If all candidates produce same pattern, entropy = 0
        let guess = Word::new("zzzzz").unwrap();
        let candidates = [
            Word::new("aaaaa").unwrap(),
            Word::new("bbbbb").unwrap(),
            Word::new("ccccc").unwrap(),
        ];
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let entropy = calculate_entropy(&guess, &candidate_refs);

        // All produce same pattern (all gray) = 0 bits
        assert!(entropy.abs() < 0.001);
    }

    #[test]
    fn calculate_entropy_perfect_split() {
        // Perfect binary split = 1 bit
        let guess = Word::new("slate").unwrap();
        let candidates = [
            Word::new("slate").unwrap(), // Perfect match
            Word::new("zzzzz").unwrap(), // No match
        ];
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let entropy = calculate_entropy(&guess, &candidate_refs);

        // Two patterns, equal probability = 1 bit
        assert!((entropy - 1.0).abs() < 0.001);
    }

    #[test]
    fn calculate_entropy_empty_candidates() {
        let guess = Word::new("crane").unwrap();
        let candidates: Vec<&Word> = vec![];

        let entropy = calculate_entropy(&guess, &candidates);
        assert!((entropy - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn group_by_pattern_works() {
        let guess = Word::new("crane").unwrap();
        let candidates = [
            Word::new("slate").unwrap(), // Different pattern than crane
            Word::new("crate").unwrap(), // Different pattern than slate
        ];
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let groups = group_by_pattern(&guess, &candidate_refs);

        // Should have 2 different patterns
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.values().sum::<usize>(), 2);
    }
}
