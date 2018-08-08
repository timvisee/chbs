//! # Correct Horse Battery Staple
//! A crate providing secure passphrase generation based on a wordlist also known as
//! [diceware].
//!
//! [![xkcd-img]][xkcd]
//!
//! The name `chbs` is short for the well known "correct horse battery staple"
//! password which originates from the [XKCD][xkcd] comic shown above.
//!
//! This library uses cryptographically secure randomization, and may be used
//! for generating secret passphrases.
//!
//! Notes:
//! * this crate is still in development, and should thus be used with care
//! * no warranty is provided for the quality of the passwords generated
//!   through this library
//!
//! TODO before stabilization which will require API changes:
//! * use secure strings
//! * allow using custom wordlists
//! * ability to configure various passphrase generation properties:
//!   * random word capitalisation
//!   * add numbers
//!   * add special characters
//!   * different separators
//!   * unique words
//! * calculate entropy
//!
//! ## Examples
//! Here are some basic examples on how to use this crate.
//!
//! Add `chbs` as dependency in your `Cargo.toml` first:
//!
//! ```toml
//! [dependencies]
//! chbs = "0.0.1"
//! ```
//!
//! Generate a passphrase using the helper function consisting of 5 words
//! ([passphrase.rs](examples/passphrase.rs)):  
//!
//! ```rust
//! extern crate chbs;
//! use chbs::passphrase;
//!
//! println!("Passphrase: {:?}", passphrase(5));
//! ```
//!
//! Run it using `cargo run --example passphrase`.
//!
//! Use a word sampler to generate an infinite number of random words
//! ([sampler.rs](examples/sampler.rs)):
//!
//! ```rust
//! extern crate chbs;
//! use chbs::word_sampler;
//!
//! let sampler = word_sampler();
//!
//! for word in sampler.take(8) {
//!     println!("Sampled word: {:?}", word);
//! }
//! ```
//!
//! Run it using `cargo run --example sampler`.
//!
//! ## License
//! This project is released under the MIT license.
//! Check out the [LICENSE](LICENSE) file for more information.
//!
//! [diceware]: https://en.wikipedia.org/wiki/Diceware
//! [xkcd]: https://xkcd.com/936/
//! [xkcd-img]: https://imgs.xkcd.com/comics/password_strength.png

#[macro_use]
extern crate derive_builder;
extern crate rand;

use std::string::ToString;

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
///
/// The word sampler is concidered cryptographically secure.
pub fn word_sampler<'a>() -> WordSampler<'a> {
    WordSampler::new(words())
}

/// Generate a secure passphrase with the given number of words.
///
/// It is recommended to use 5 or more words when possible.
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

/// Generate a secure passphrase with the given number of words.
///
/// It is recommended to use 5 or more words when possible.
///
/// # Panics
///
/// The number of words must at least be 1.
pub fn passphrase_config<C>(words: usize, config: &C) -> String
    where
        C: Separator + Capitalize,
{
    if words == 0 {
        panic!("it is not allowed to generate a passphrase with 0 words");
    }

    // Build a randomizer used while building the passphrase
    // TODO: use a shared randomizer
    let mut rng = thread_rng();

    word_sampler()
        .take(words)
        .map(|word| {
            let mut word = word.to_owned();
            config.capitalize(&mut word, &mut rng);
            word
        })
        .fold(String::new(), |mut phrase, word| {
            // Append a separator
            if !phrase.is_empty() {
                phrase += config.yield_separator();
            }

            // Append the word
            phrase += &word;
            phrase
        })
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

/// Something that provides separators to use between passphrase words.
pub trait Separator {
    /// Yield a separator to use between passphrase words.
    /// Each yielded separator must only be used once.
    fn yield_separator(&self) -> &str;
}

/// Something that provides capitalization for passphrase words.
///
/// Each word is processed through the `capitalize` function,
/// which applies the capitalization as specified by the provider.
/// The word is mutated in-place.
pub trait Capitalize {
    /// Capitalize the given word as specified by this provider.
    fn capitalize<R: Rng>(&self, word: &mut String, rng: &mut R);
}

/// A simple configuration for passphrase generation.
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct SimpleConfig {
    /// The separator string to use between passphrase words.
    separator: String,

    /// Whether to capitalize the first characters of words.
    capitalize_first: Occurrence,

    /// Whether to capitalize whole words.
    capitalize_words: Occurrence,
}

impl Default for SimpleConfig {
    fn default() -> SimpleConfig {
        SimpleConfig {
            separator: " ".into(),
            capitalize_first: Occurrence::Sometimes,
            capitalize_words: Occurrence::Never,
        }
    }
}

impl Separator for SimpleConfig {
    fn yield_separator(&self) -> &str {
        &self.separator
    }
}

impl Capitalize for SimpleConfig {
    fn capitalize<R: Rng>(&self, word: &mut String, rng: &mut R) {
        // Do not do anything if emtpy
        if word.is_empty() {
            return;
        }

        // Capitalize first characters
        if self.capitalize_first.yield_occurrence(rng) {
            let first = word
                .chars()
                .map(|c| c.to_uppercase().to_string())
                .next()
                .unwrap_or_else(|| String::new());
            let rest: String = word
                .chars()
                .skip(1)
                .collect();
            *word = first + &rest;
        }

        // Capitalize whole words
        if self.capitalize_words.yield_occurrence(rng) {
            *word = word.to_uppercase();
        }
    }
}

/// A definition of how often something occurs.
#[derive(Copy, Clone, Debug)]
pub enum Occurrence {
    /// This occurs all the time.
    Always,

    /// This sometimes (cryptographically randomly) occurs.
    Sometimes,

    /// This never occurs.
    Never,
}

impl Occurrence {
    /// Yield an occurrence.
    pub fn yield_occurrence<R: Rng>(&self, rng: &mut R) -> bool {
        match self {
            Occurrence::Always => true,
            Occurrence::Never => false,
            Occurrence::Sometimes => rng.gen(),
        }
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
