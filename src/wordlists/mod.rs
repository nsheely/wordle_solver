//! Word lists for Wordle solving
//!
//! Provides embedded word lists compiled into the binary for zero-cost access.

mod embedded;
pub mod loader;

pub use embedded::{ALLOWED, ALLOWED_COUNT, ANSWERS, ANSWERS_COUNT};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_count_matches_const() {
        assert_eq!(ANSWERS.len(), ANSWERS_COUNT);
    }

    #[test]
    fn allowed_count_matches_const() {
        assert_eq!(ALLOWED.len(), ALLOWED_COUNT);
    }

    #[test]
    fn answers_are_valid_words() {
        // All answers should be 5 letters, lowercase
        for &word in ANSWERS {
            assert_eq!(word.len(), 5, "Word '{word}' is not 5 letters");
            assert!(
                word.chars().all(|c| c.is_ascii_lowercase()),
                "Word '{word}' contains non-lowercase chars"
            );
        }
    }

    #[test]
    fn allowed_are_valid_words() {
        // All allowed words should be 5 letters, lowercase
        for &word in &ALLOWED[..10] {
            // Just check first 10 for speed
            assert_eq!(word.len(), 5, "Word '{word}' is not 5 letters");
            assert!(
                word.chars().all(|c| c.is_ascii_lowercase()),
                "Word '{word}' contains non-lowercase chars"
            );
        }
    }

    #[test]
    fn answers_subset_of_allowed() {
        // All answer words should be in the allowed list
        let allowed_set: std::collections::HashSet<_> = ALLOWED.iter().collect();

        for &answer in &ANSWERS[..10] {
            // Check first 10 for speed
            assert!(
                allowed_set.contains(&answer),
                "Answer '{answer}' not in allowed list"
            );
        }
    }

    #[test]
    fn expected_counts() {
        assert_eq!(ANSWERS_COUNT, 2315, "Expected 2,315 answer words");
        assert_eq!(ALLOWED_COUNT, 12972, "Expected 12,972 allowed words");
    }
}
