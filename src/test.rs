mod wordle;

#[cfg(test)]
mod tests {
    use crate::wordle::{Word, Color, compute_filter, compute_best_query, get_query_bank, get_solution_bank};

    fn test_compute_filter_helper(query: &str, secret: &str) -> [Color; 5] {
        compute_filter(&Word::new(&query), &Word::new(&secret)).colors
    }

    #[test]
    fn test_compute_filter() {
        assert_eq!(test_compute_filter_helper("oooll", "llool"), [Color::YELLOW, Color::GRAY, Color::GREEN, Color::YELLOW, Color::GREEN]);
        assert_eq!(test_compute_filter_helper("alaap", "pause"), [Color::YELLOW, Color::GRAY, Color::GRAY, Color::GRAY, Color::YELLOW]);
        assert_eq!(test_compute_filter_helper("bench", "bench"), [Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN]);
    }

    #[test]
    fn test_compute_best_query() {
        let word_bank = get_query_bank();
        let secret_candidates = get_solution_bank();
        assert_eq!(compute_best_query(&word_bank, &secret_candidates), Word::new("aesir"));
    }
}