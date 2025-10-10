//! Formatting utilities for terminal output

use crate::core::Pattern;

/// Format a pattern as emoji string
#[must_use]
pub fn pattern_to_emoji(pattern: Pattern) -> String {
    let mut result = String::with_capacity(5);
    let mut val = pattern.value();

    for _ in 0..5 {
        let digit = val % 3;
        result.push(match digit {
            0 => '⬜', // Gray
            1 => '🟨', // Yellow
            2 => '🟩', // Green
            _ => unreachable!(),
        });
        val /= 3;
    }

    result
}

/// Create a progress bar string
#[must_use]
pub fn create_progress_bar(value: f64, max: f64, width: usize) -> String {
    // Cast is safe: values are clamped to [0, width]
    let filled = ((value / max) * width as f64) as usize;
    let filled = filled.min(width);

    format!("{}{}", "█".repeat(filled), "░".repeat(width - filled))
}

/// Format entropy as a colored bar
#[must_use]
pub fn entropy_bar(entropy: f64, width: usize) -> String {
    let max_entropy = 6.0; // Roughly log2(64)
    create_progress_bar(entropy, max_entropy, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_to_emoji_all_gray() {
        let pattern = Pattern::new(0); // All gray
        let emoji = pattern_to_emoji(pattern);
        assert_eq!(emoji, "⬜⬜⬜⬜⬜");
    }

    #[test]
    fn pattern_to_emoji_all_green() {
        let pattern = Pattern::PERFECT;
        let emoji = pattern_to_emoji(pattern);
        assert_eq!(emoji, "🟩🟩🟩🟩🟩");
    }

    #[test]
    fn progress_bar_empty() {
        let bar = create_progress_bar(0.0, 100.0, 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn progress_bar_full() {
        let bar = create_progress_bar(100.0, 100.0, 10);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn progress_bar_half() {
        let bar = create_progress_bar(50.0, 100.0, 10);
        assert_eq!(bar, "█████░░░░░");
    }
}
