//! # Correct Horse Battery Staple
//! A crate for secure passphrase generation based on a wordlist.
//!
//! ## Generating a passphrase
//! To generate a secure passphrase, the helper function `passphrase` can be
//! used. Specify the number of words the passphrase must consist of,
//! and generate a passphrase string.
//!
//! ```rust
//! extern crate chbs;
//! use chbs::passphrase;
//!
//! println!("Passphrase: {:?}", passphrase(5));
//! ```
//!
//! ## Generate random words
//! A word sampler can be created to generate an infinite number of
//! cryptographically secure random words based on the included wordlist.
//!
//! ```rust
//! extern crate chbs;
//! use chbs::word_sampler;
//!
//! let mut sampler = word_sampler();
//!
//! for i in 0..8 {
//!     println!(
//!         "Sampled word #{}: {:?}",
//!         i,
//!         sampler.next().unwrap(),
//!     );
//! }
//! ```

extern crate rand;

use rand::{
    distributions::Uniform,
    prelude::*,
    rngs::ThreadRng,
    thread_rng,
};

/// A static wordlist to use.
const WORDLIST: &'static str = include_str!("../res/eff_large_wordlist.txt");

/// Build a vector of words based on a wordlist to use for passphrase
/// generation.
///
/// The included wordlist is prefixed with dice numbers, each line ends with
/// an actual word. This function grabs the last alphabetic word from each
/// line which are then collected into a vector.
pub fn words<'a>() -> Vec<&'a str> {
    WORDLIST
        .lines()
        .filter_map(|line| line
            .trim_right()
            .rsplit_terminator(char::is_whitespace)
            .next()
        )
        .collect()
}

/// Build a word sampler which is an iterator that randomly samples words from
/// the included wordlist.
/// The word sampler is concidered cryptographically secure.
pub fn word_sampler<'a>() -> WordSampler<'a> {
    WordSampler::new(words())
}

/// Generate a secure passphrase with the given number of words.
/// It is recommended to use 4 or more words when possible.
///
/// # Panics
/// 
/// The number of words must at least be 1.
pub fn passphrase(words: usize) -> String {
    if words == 0 {
        panic!("it is not allowed to generate a passphrase with 0 words");
    }

    word_sampler()
        .take(words)
        .collect::<Vec<_>>()
        .join(" ")
}

/// A word sampler iterator that provides random words from a given wordlist.
/// The randomization is concidered cryptographically secure.
///
/// As much words as needed may be pulled from this iterator.
pub struct WordSampler<'a> {
    /// List of words that is used for sampling.
    words: Vec<&'a str>,

    /// Random distribution used for sampling.
    distribution: Uniform<usize>,

    /// Random number generator used for sampling.
    rng: ThreadRng,
}

impl<'a> WordSampler<'a> {
    /// Build a new word sampler which samples the given word list.
    pub fn new(words: Vec<&'a str>) -> WordSampler<'a> {
        WordSampler {
            distribution: Uniform::new(0, words.len()),
            words,
            rng: thread_rng(),
        }
    }
}

impl<'a> Iterator for WordSampler<'a> {
    type Item = &'a str;

    /// Sample the next random word.
    /// This iterator is infinite and always returns some word.
    fn next(&mut self) -> Option<&'a str> {
        let i = self.rng.sample(self.distribution);
        Some(self.words[i])
    }
}

#[cfg(test)]
mod tests {
    use {passphrase, words, WordSampler, word_sampler};

    /// How many times to iterate for small or infinite tests.
    const ITERS: usize = 32;

    /// We must have words to use.
    #[test]
    fn any_words() {
        assert!(!words().is_empty());
    }

    /// Words must at least be 3 characters long.
    #[test]
    fn no_short_words() {
        assert!(words().iter().all(|word| word.len() >= 3));
    }

    /// Words must only contain alphabetical characters or a hyphen.
    #[test]
    fn all_alpha_hyphen_words() {
        assert!(
            words().iter().all(|word|
                word.chars().all(|c|
                    c.is_alphabetic() || c == '-'
                )
            )
        );
    }

    /// Generating a passphrase must produce the correct number of words.
    #[test]
    fn passphrase_words() {
        for i in 1..=ITERS {
            assert_eq!(passphrase(i).split(char::is_whitespace).count(), i);
        }
    }

    /// Generating a passphrase with 0 words should panic.
    #[test]
    #[should_panic]
    fn empty_passphrase_panic() {
        passphrase(0);
    }

    /// Ensure a word sampler is able to produce words.:w
    /// This test only covers a limited number of checks as the sampler
    /// itself is infinite.
    #[test]
    fn word_sampler_produces() {
        assert_eq!(
            word_sampler()
                .take(ITERS)
                .count(),
            ITERS,
        );
    }

    /// Ensure a word sampler produces words that are in the word list.
    /// This test only covers a limited number of checks as the sampler
    /// itself is infinite.
    #[test]
    fn word_sampler_known_words() {
        // Get a list of words, and build a sampler
        let words = words();
        let sampler = WordSampler::new(words.clone());

        assert_eq!(
            sampler
                .take(ITERS)
                .filter(|word| words.contains(word))
                .count(),
            ITERS,
        );
    }
}
