//! Word analysis command
//!
//! Analyzes the entropy and information content of a specific word.

use crate::core::Word;
use crate::solver::entropy::calculate_entropy;

/// Result of analyzing a word
pub struct AnalysisResult {
    pub word: String,
    pub entropy: f64,
    pub expected_reduction: f64,
    pub expected_remaining: f64,
    pub total_candidates: usize,
}

/// Analyze the entropy of a word against a set of candidates
///
/// # Errors
///
/// Returns an error if:
/// - The word is invalid (not 5 letters or contains non-ASCII)
/// - The word is not in the provided word list
pub fn analyze_word(
    word: &str,
    all_words: &[Word],
    candidates: &[Word],
) -> Result<AnalysisResult, String> {
    let word_obj = Word::new(word).map_err(|e| format!("Invalid word: {e}"))?;

    // Check if word exists in all_words
    if !all_words.iter().any(|w| w.text() == word_obj.text()) {
        return Err(format!("Word '{word}' not in word list"));
    }

    let candidate_refs: Vec<&Word> = candidates.iter().collect();
    let entropy = calculate_entropy(&word_obj, &candidate_refs);

    let total_candidates = candidates.len();
    let expected_reduction = entropy.exp2();
    let expected_remaining = total_candidates as f64 / expected_reduction;

    Ok(AnalysisResult {
        word: word.to_string(),
        entropy,
        expected_reduction,
        expected_remaining,
        total_candidates,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wordlists::ANSWERS;
    use crate::wordlists::loader::words_from_slice;

    #[test]
    fn analyze_valid_word() {
        let words = words_from_slice(&ANSWERS[..100]);

        // Use a word we know is in the first 100
        let result = analyze_word("aback", &words, &words).unwrap();

        assert_eq!(result.word, "aback");
        assert!(result.entropy > 0.0);
        assert!(result.expected_reduction >= 1.0);
        assert_eq!(result.total_candidates, 100);
    }

    #[test]
    fn analyze_invalid_word() {
        let words = words_from_slice(&ANSWERS[..100]);

        let result = analyze_word("zzzzz", &words, &words);
        assert!(result.is_err());
    }

    #[test]
    fn entropy_properties() {
        let words = words_from_slice(&ANSWERS[..100]);

        let result = analyze_word("aback", &words, &words).unwrap();

        // Entropy should be bounded
        assert!(result.entropy >= 0.0);
        assert!(result.entropy <= (words.len() as f64).log2());

        // Expected remaining should be sensible
        assert!(result.expected_remaining >= 0.0);
        assert!(result.expected_remaining <= words.len() as f64);
    }
}
