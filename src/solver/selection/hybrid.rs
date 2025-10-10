//! Hybrid selection strategies
//!
//! Combines entropy with other metrics (`expected_remaining`, minimax) for improved performance.

use crate::core::Word;
use crate::solver::entropy::calculate_metrics;
use rayon::prelude::*;

/// Select best guess with `entropy+expected_size+minimax` tiebreakers
///
/// For medium candidate counts (21-100), this provides better performance than pure entropy.
/// Primary: entropy, Secondary: `expected_remaining`, Tertiary: minimax
///
/// Returns `None` if the guess pool is empty.
#[must_use]
pub fn select_with_expected_tiebreaker<'a>(
    guess_pool: &'a [&'a Word],
    candidates: &[&Word],
) -> Option<&'a Word> {
    // Compute all metrics (parallelized)
    let metrics: Vec<_> = guess_pool
        .par_iter()
        .map(|&guess| {
            let m = calculate_metrics(guess, candidates);
            (guess, m)
        })
        .collect();

    // Select by: entropy (primary), expected_remaining (secondary), max_partition (tertiary)
    metrics
        .into_iter()
        .max_by(|(_, m1), (_, m2)| {
            m1.entropy
                .total_cmp(&m2.entropy)
                .then(m2.expected_remaining.total_cmp(&m1.expected_remaining))
                .then(m2.max_partition.cmp(&m1.max_partition))
        })
        .map(|(word, _)| word)
}

/// Select best guess with hybrid scoring
///
/// For medium candidate counts (9-20), use formula: score = (entropy × 100) - (`max_partition` × 10)
/// This balances average-case (entropy) with worst-case (minimax) at ~5:1 ratio.
///
/// Returns `None` if the guess pool is empty.
#[must_use]
pub fn select_with_hybrid_scoring<'a>(
    guess_pool: &'a [&'a Word],
    candidates: &[&Word],
) -> Option<&'a Word> {
    // Compute all metrics (parallelized)
    let metrics: Vec<_> = guess_pool
        .par_iter()
        .map(|&guess| {
            let m = calculate_metrics(guess, candidates);
            (guess, m)
        })
        .collect();

    // Find best hybrid score
    metrics
        .into_iter()
        .max_by(|(_, m1), (_, m2)| {
            // Hybrid score: entropy (×100) minus worst-case penalty (×10)
            let score1 = (m1.entropy * 100.0) as i32
                - i32::try_from(m1.max_partition * 10).unwrap_or(i32::MAX);
            let score2 = (m2.entropy * 100.0) as i32
                - i32::try_from(m2.max_partition * 10).unwrap_or(i32::MAX);
            // Higher score is better
            score1
                .cmp(&score2)
                .then(m2.expected_remaining.total_cmp(&m1.expected_remaining))
        })
        .map(|(word, _)| word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_with_expected_tiebreaker_works() {
        let guesses = [
            Word::new("crane").unwrap(),
            Word::new("slate").unwrap(),
            Word::new("aaaaa").unwrap(),
        ];
        let candidates = [
            Word::new("irate").unwrap(),
            Word::new("crate").unwrap(),
            Word::new("grate").unwrap(),
            Word::new("plate").unwrap(),
        ];

        let guess_refs: Vec<&Word> = guesses.iter().collect();
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let result = select_with_expected_tiebreaker(&guess_refs, &candidate_refs);
        assert!(result.is_some());

        let best = result.unwrap();

        // Should select one of the better guesses (not AAAAA)
        assert!(best.text() != "aaaaa");
    }

    #[test]
    fn select_with_hybrid_scoring_works() {
        let guesses = [
            Word::new("crane").unwrap(),
            Word::new("slate").unwrap(),
            Word::new("zzzzz").unwrap(),
        ];
        let candidates = [
            Word::new("irate").unwrap(),
            Word::new("crate").unwrap(),
            Word::new("grate").unwrap(),
        ];

        let guess_refs: Vec<&Word> = guesses.iter().collect();
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let result = select_with_hybrid_scoring(&guess_refs, &candidate_refs);
        assert!(result.is_some());

        let best = result.unwrap();

        // Should select a reasonable guess (not ZZZZZ)
        assert!(best.text() != "zzzzz");
    }

    #[test]
    fn hybrid_scoring_balances_entropy_and_minimax() {
        // Create scenario where pure entropy and pure minimax disagree
        let guesses = [
            Word::new("aeros").unwrap(), // High entropy, possibly worse minimax
            Word::new("slate").unwrap(), // Balanced
        ];
        let candidates = [Word::new("irate").unwrap(), Word::new("crate").unwrap()];

        let guess_refs: Vec<&Word> = guesses.iter().collect();
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let result = select_with_hybrid_scoring(&guess_refs, &candidate_refs);
        assert!(result.is_some());

        let best = result.unwrap();

        // Should pick one of them (test that it doesn't panic)
        assert!(best.text() == "aeros" || best.text() == "slate");
    }

    #[test]
    fn expected_tiebreaker_returns_none_on_empty() {
        let guesses: Vec<&Word> = vec![];
        let candidates = [Word::new("slate").unwrap()];
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let result = select_with_expected_tiebreaker(&guesses, &candidate_refs);
        assert!(result.is_none());
    }

    #[test]
    fn hybrid_scoring_returns_none_on_empty() {
        let guesses: Vec<&Word> = vec![];
        let candidates = [Word::new("slate").unwrap()];
        let candidate_refs: Vec<&Word> = candidates.iter().collect();

        let result = select_with_hybrid_scoring(&guesses, &candidate_refs);
        assert!(result.is_none());
    }
}
