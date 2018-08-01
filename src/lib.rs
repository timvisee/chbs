/// A static wordlist to use.
const WORDLIST: &'static str = include_str!("../res/eff_large_wordlist.txt");

/// Build a vector of words based on a wordlist to use for passphrase
/// generation.
///
/// The included wordlist is prefixed with dice numbers, each line ends with
/// an actual word. This function grabs the last alphabetic word from each
/// line which are then collected into a vector.
fn words<'a>() -> Vec<&'a str> {
    WORDLIST
        .lines()
        .filter_map(|line| line
            .trim_right()
            .rsplit_terminator(char::is_whitespace)
            .next()
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use words;

    /// We must have words to use.
    #[test]
    fn any_words() {
        assert!(!words().is_empty());
    }

    /// We must not have any empty words.
    #[test]
    fn no_empty_words() {
        assert!(words().iter().all(|word| !word.is_empty()));
    }
}
