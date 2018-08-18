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
//! Generate a passphrase with the default configuration using the helper
//! function consisting of 5 words
//! ([passphrase.rs](examples/passphrase.rs)):  
//!
//! ```rust
//! extern crate chbs;
//! use chbs::{config, passphrase};
//!
//! println!("Passphrase: {:?}", passphrase(&config()));
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

pub mod prelude;

use std::string::ToString;

use rand::{
    distributions::Uniform,
    prelude::*,
    rngs::ThreadRng,
    thread_rng,
};

/// A static wordlist to use.
const WORDLIST: &'static str = include_str!("../res/eff_large_wordlist.txt");

/// The default number of words the passphrase will consist of.
const DEFAULT_WORDS: usize = 5;

/// The default separator used between passphrase words.
const DEFAULT_SEPARATOR: &'static str = " ";

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

/// Generate a secure passphrase based on the given configuration.
///
/// It is recommended to use 5 or more words when possible.
///
/// # Panics
///
/// The number of words must at least be 1.
pub fn passphrase<C>(config: &C) -> String
    where
        C: WordCount + Separator + Capitalize,
{
    // Yield the word count
    let words = config.yield_word_count();
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

/// Build a default basic configuration to use for passphrase generation.
pub fn config() -> BasicConfig {
    BasicConfig::default()
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





/// This trait provides configuration essentials used when generating a passphrase.
///
/// It provides essential functions that define a list of word processors, phrase processors and a
/// phrase builder to generate a passphrase based on a list of words.
///
/// This trait may be implemented on any struct to provide your own desired configuration features.
// TODO: Create a config struct (scheme) with a fixed set of word and phrase processors for consistency.
pub trait Config {
    /// Get the word generator that is used to generate an ordered set of passphrase words, which
    /// will then be processed into a final passphrase by other components.
    fn word_generator(&self) -> Box<dyn WordGenerator>;

    /// Get a list of all word processors that are part of this configuration.
    /// Each word processor is applied to each word in the passphrase, in the order they are
    /// returned in.
    fn word_processors(&self) -> Vec<Box<dyn WordProcessor>>;

    /// Get the phrase builder. Which is used to combine a set of passphrase words into a final
    /// passphrase.
    ///
    /// Each given word has already been processed by all word processors.
    /// The phrase builder should glue these words together with the proper word eparators.
    fn phrase_builder(&self) -> Box<dyn PhraseBuilder>;

    /// Get a list of all phrase processors that are part of this configuration.
    /// Each phrase processor is applied to the whole passphrase built by the passphrase builder,
    /// in the order they are returned in.
    ///
    /// If no phrase processor is returned the phrase isn't modified after building it with the
    /// phrase builder.
    fn phrase_processors(&self) -> Vec<Box<dyn PhraseProcessor>>;

    /// Generate a single passphrase.
    // TODO: provide a randomness source
    fn generate(&self) -> String {
        // Generate the passphrase words
        let mut words = self.word_generator().generate_words();

        // Run the passphrase words through the word processors
        for processor in self.word_processors() {
            words = words
                .into_iter()
                .map(|word| processor.process_word(word))
                .collect();
        }

        // Build the passphrase
        let mut phrase = self.phrase_builder().build_phrase(words);

        // Run the phrase through the passphrase processors
        for processor in self.phrase_processors() {
            phrase = processor.process_phrase(phrase);
        }

        phrase
    }
}

/// Get the entropy value for the current component, whether that is a word processor, a phrase
/// builder or something else.
///
/// TODO: properly describe what entropy is here.
pub trait Entropy {
    /// Get the entropy value for this whole component.
    /// The returned entropy value may be accumulated from various internal entropy sources.
    ///
    /// See the documentation on [Entropy](Entropy) for details on what entropy is and how it
    /// should be calculated.
    /// If this component does not have any effect on passphrase entropy `1` should be returned.
    /// TODO: should this be an integer, or a big integer?
    fn entropy(&self) -> f64;
}

/// A component that provides functionallity to generate passphrase words.
/// On generation, an ordered list of passphrase words is returned that will be used in the
/// password.
pub trait WordGenerator: Entropy {
    /// Generate an ordered set of passphrase words to use in a password.
    fn generate_words(&self) -> Vec<String>;
}

/// Something that provides logic to process each passphrase word.
/// This could be used to build a processor for word capitalization.
pub trait WordProcessor: Entropy {
    /// Process the given word.
    fn process_word(&self, word: String) -> String;
}

/// Something that provides logic to combine a list of passphrase words into a passphrase.
pub trait PhraseBuilder: Entropy {
    /// Build the passphrase from the given words, and combine them in one final passphrase.
    fn build_phrase(&self, words: Vec<String>) -> String;
}

/// Something that provides logic to process a passphrase as a whole.
pub trait PhraseProcessor: Entropy {
    /// Process the given passphrase as a whole.
    /// The processed passphrase is returned.
    fn process_phrase(&self, phrase: String) -> String;
}





/// A passphrase word generator that generates a fixed number of passphrase words.
/// TODO: configure the wordlist to use.
pub struct FixedGenerator {
    /// The number of passphrase words to generate.
    words: usize,
}

impl FixedGenerator {
    /// Construct a new generator.
    ///
    /// The number of `words` to generate must be specified.
    /// It is recommended to use at least 5 passphrase words with a wordlist of at least
    /// 7776 (6^5) words.
    ///
    /// # Panics
    ///
    /// This panics when the given number of `words` is `0`.
    pub fn new(words: usize) -> Self {
        if words == 0 {
            panic!("cannot create passphrase word generator, word count may not be zero");
        }

        Self {
            words,
        }
    }
}

impl Entropy for FixedGenerator {
    fn entropy(&self) -> f64 {
        // TODO: multiply word list size by the word count
        (7776 * self.words) as f64
    }
}

impl WordGenerator for FixedGenerator {
    fn generate_words(&self) -> Vec<String> {
        word_sampler()
            .take(self.words)
            .map(|word| word.to_owned())
            .collect()
    }
}

/// A basic passphrase builder, which combines passphrase words into a passphrase with a static
/// separator.
pub struct BasicPhraseBuilder {
    /// The separator that is used.
    separator: String,
}

impl BasicPhraseBuilder {
    pub fn new(separator: String) -> Self {
        Self {
            separator,
        }
    }
}

impl Entropy for BasicPhraseBuilder {
    fn entropy(&self) -> f64 {
        1.0
    }
}

impl PhraseBuilder for BasicPhraseBuilder {
    fn build_phrase(&self, words: Vec<String>) -> String {
        words.join(&self.separator)
    }
}





/// A simple configuration for passphrase generation.
#[derive(Builder, Clone, Debug)]
#[builder(default, setter(into))]
pub struct BasicConfig {
    /// The number of words the passphrase must consist of.
    pub words: usize,

    /// The separator string to use between passphrase words.
    pub separator: String,

    /// Whether to capitalize the first characters of words.
    pub capitalize_first: Occurrence,

    /// Whether to capitalize whole words.
    pub capitalize_words: Occurrence,
}

impl Default for BasicConfig {
    fn default() -> BasicConfig {
        BasicConfig {
            words: DEFAULT_WORDS,
            separator: DEFAULT_SEPARATOR.into(),
            capitalize_first: Occurrence::Sometimes,
            capitalize_words: Occurrence::Never,
        }
    }
}

impl WordCount for BasicConfig {
    fn yield_word_count(&self) -> usize {
        self.words
    }
}

impl Separator for BasicConfig {
    fn yield_separator(&self) -> &str {
        &self.separator
    }
}

impl Capitalize for BasicConfig {
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

/// Something that provides the number of words a passphrase must consist of.
pub trait WordCount {
    /// Yield the number of words the passphrase should consist of.
    ///
    /// This function must be called when generating a passphrase to
    /// determine the number of words, then the words should be yielded
    /// and processed.
    fn yield_word_count(&self) -> usize;
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

/// Allow easy `Occurrence` selection of `Always` and `Never` from a boolean.
impl From<bool> for Occurrence {
    fn from(b: bool) -> Occurrence {
        match b {
            true => Occurrence::Always,
            false => Occurrence::Never,
        }
    }
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
    use {config, passphrase, words, WordSampler, word_sampler};

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
        let mut config = config();

        for words in 1..=ITERS {
            config.words = words;
            assert_eq!(
                passphrase(&config).split(char::is_whitespace).count(),
                words,
            );
        }
    }

    /// Generating a passphrase with 0 words should panic.
    #[test]
    #[should_panic]
    fn empty_passphrase_panic() {
        let mut config = config();
        config.words = 0;

        passphrase(&config);
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
