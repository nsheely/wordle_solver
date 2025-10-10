//! Word list loading utilities
//!
//! Provides functions to load word lists from files or use embedded constants.

use crate::core::Word;
use std::fs;
use std::io;
use std::path::Path;

/// Load words from a file
///
/// Returns a vector of valid Word instances, skipping any invalid entries.
///
/// # Errors
///
/// Returns an I/O error if the file cannot be read or opened.
///
/// # Examples
/// ```no_run
/// use wordle_solver::wordlists::loader::load_from_file;
///
/// let words = load_from_file("data/answers.txt").unwrap();
/// println!("Loaded {} words", words.len());
/// ```
pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Word>> {
    let content = fs::read_to_string(path)?;

    let words = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Word::new(trimmed).ok()
            }
        })
        .collect();

    Ok(words)
}

/// Convert embedded string slice to Word vector
///
/// # Examples
/// ```
/// use wordle_solver::wordlists::loader::words_from_slice;
/// use wordle_solver::wordlists::ANSWERS;
///
/// let words = words_from_slice(ANSWERS);
/// assert_eq!(words.len(), ANSWERS.len());
/// ```
#[must_use]
pub fn words_from_slice(slice: &[&str]) -> Vec<Word> {
    slice.iter().filter_map(|&s| Word::new(s).ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_from_slice_converts_valid_words() {
        let input = &["crane", "slate", "irate"];
        let words = words_from_slice(input);

        assert_eq!(words.len(), 3);
        assert_eq!(words[0].text(), "crane");
        assert_eq!(words[1].text(), "slate");
        assert_eq!(words[2].text(), "irate");
    }

    #[test]
    fn words_from_slice_skips_invalid() {
        let input = &["crane", "toolong", "abc", "slate"];
        let words = words_from_slice(input);

        // Only "crane" and "slate" are valid 5-letter words
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text(), "crane");
        assert_eq!(words[1].text(), "slate");
    }

    #[test]
    fn words_from_slice_empty() {
        let input: &[&str] = &[];
        let words = words_from_slice(input);
        assert_eq!(words.len(), 0);
    }

    #[test]
    fn load_from_embedded_answers() {
        use crate::wordlists::ANSWERS;

        let words = words_from_slice(ANSWERS);
        assert_eq!(words.len(), ANSWERS.len());
    }
}
