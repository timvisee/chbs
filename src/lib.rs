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

// TODO: create a fixed scheme configuration for consistency
// TODO: create an entropy wrapper for easier calculations
// TODO: create wordlist struct
// TODO: use wordlist in word sampler
// TODO: create word generator using sampler or random generator

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

///// Generate a secure passphrase based on the given configuration.
/////
///// It is recommended to use 5 or more words when possible.
/////
///// # Panics
/////
///// The number of words must at least be 1.
//pub fn passphrase<C>(config: &C) -> String
//    where
//        C: WordCount + Separator + Capitalize,
//{
//    // Yield the word count
//    let words = config.yield_word_count();
//    if words == 0 {
//        panic!("it is not allowed to generate a passphrase with 0 words");
//    }

//    // Build a randomizer used while building the passphrase
//    // TODO: use a shared randomizer
//    let mut rng = thread_rng();

//    word_sampler()
//        .take(words)
//        .map(|word| {
//            let mut word = word.to_owned();
//            config.capitalize(&mut word, &mut rng);
//            word
//        })
//        .fold(String::new(), |mut phrase, word| {
//            // Append a separator
//            if !phrase.is_empty() {
//                phrase += config.yield_separator();
//            }

//            // Append the word
//            phrase += &word;
//            phrase
//        })
//}

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





/// A passphrase generation scheme.
///
/// The scheme defines how passphrases should be generated, and can be directly used to so.
/// This scheme holds various components used during generation to modify and combine passphrase
/// parts or words.
///
/// It is recommended to use a configuration struct to confige and build a specific `Scheme`
/// instead of setting one up manually. See: [`BasicConfig`](BasicConfig).
///
/// A scheme cannot be modified after creation, to ensure passphrase generation and calculating
/// entropy is consistent.
///
/// # Components
///
/// The following components are part of this scheme and the passphrase generation process:
///
/// - The word generator is used once for each passphrase to generate, and provides a set of words
///   to use for that specific phrase. The generator internally samples a known wordlist or
///   generates randomized strings depending on how it is configured.
/// - A set of word processors is used to modify each passphrase word from the generated set, to
///   randomize capitalization, to add special characters and more depending on their
///   configuration. Each processor is applied once to each phrase word in the specified order.
///   If no word processor is available, the words are kept intact.
/// - The phrase builder combines the set of now modified passphrase words into a full passphrase,
///   the builder separates the words with a space or anything else depending on it's
///   configuration.
/// - A set of phrase processors is used to modify the full passphrase that is now combined. They
///   may be used for further modifications with full control over the phrase. If no phrase
///   processor is available, the phrase is kept intact.
pub struct Scheme {
    /// A word generator, which generates sets of words to use in the passphrase.
    word_generator: Box<dyn WordGenerator>,

    /// A set of word processors to apply to each passphrase word.
    word_processors: Vec<Box<dyn WordProcessor>>,

    /// A phrase builder that builds a passphrase out of a processed set of passphrase words.
    phrase_builder: Box<dyn PhraseBuilder>,

    /// A set of phrase processors to apply to each passphrase.
    phrase_processors: Vec<Box<dyn PhraseProcessor>>,
}

impl Scheme {
    /// Construct a new password scheme based on the given set of components.
    pub fn new(
        word_generator: Box<dyn WordGenerator>,
        word_processors: Vec<Box<dyn WordProcessor>>,
        phrase_builder: Box<dyn PhraseBuilder>,
        phrase_processors: Vec<Box<dyn PhraseProcessor>>,
    ) -> Self {
        Self {
            word_generator,
            word_processors,
            phrase_builder,
            phrase_processors,
        }
    }

    /// Build a configuration based on the given object.
    pub fn from<S: ToScheme>(config: &S) -> Self {
        config.to_scheme()
    }

    /// Generate a single passphrase.
    fn generate(&self) -> String {
        // Generate the passphrase words
        let mut words = self.word_generator.generate_words();

        // Run the passphrase words through the word processors
        for p in &self.word_processors {
            words = words
                .into_iter()
                .map(|w| p.process_word(w))
                .collect();
        }

        // Build the passphrase
        let mut phrase = self.phrase_builder.build_phrase(words);

        // Run the phrase through the passphrase processors
        for p in &self.phrase_processors {
            phrase = p.process_phrase(phrase);
        }

        phrase
    }

    /// Calculate the entropy this configuration produces.
    ///
    /// See the documentation on [Entropy](Entropy) for details on what entropy is and how it
    /// should be calculated.
    fn entropy(&self) -> f64 {
        self.word_generator.entropy()
            * self.word_processors.iter().map(|p| p.entropy()).product::<f64>()
            * self.phrase_builder.entropy()
            * self.phrase_processors.iter().map(|p| p.entropy()).product::<f64>()
    }
}

/// A trait providing an interface to build a password scheme based on some sort of configuration.
pub trait ToScheme {
    /// Build a password scheme based on configuration in this object.
    fn to_scheme(&self) -> Scheme;
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
    /// TODO: use an entropy struct to track the entropy value
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
    /// Process the given `word`.
    fn process_word(&self, word: String) -> String;
}

/// Something that provides logic to combine a list of passphrase words into a passphrase.
pub trait PhraseBuilder: Entropy {
    /// Build the passphrase from the given words, and combine them in one final passphrase.
    fn build_phrase(&self, words: Vec<String>) -> String;
}

/// Something that provides logic to process a passphrase as a whole.
pub trait PhraseProcessor: Entropy {
    /// Process the given `phrase` as a whole.
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

/// A word processor component that is used to randomly capitalize the first character of
/// passphrase words, or the whole word at once.
pub struct WordCapitalizer {
    /// Whether to capitalize the first characters of words.
    first: Occurrence,

    /// Whether to capitalize whole words.
    all: Occurrence,
}

impl WordCapitalizer {
    pub fn new(first: Occurrence, all: Occurrence) -> Self {
        Self {
            first,
            all,
        }
    }
}

impl Entropy for WordCapitalizer {
    fn entropy(&self) -> f64 {
        self.first.entropy() * self.all.entropy()
    }
}

impl WordProcessor for WordCapitalizer {
    fn process_word(&self, mut word: String) -> String {
        if word.is_empty() {
            return word;
        }

        let mut rng = thread_rng();

        // Capitalize the first character
        if self.first.yield_occurrence(&mut rng) {
            let first = word
                .chars()
                .map(|c| c.to_uppercase().to_string())
                .next()
                .unwrap_or_else(|| String::new());
            let rest: String = word
                .chars()
                .skip(1)
                .collect();
            word = first + &rest;
        }

        // Capitalize whole words
        if self.all.yield_occurrence(&mut rng) {
            word = word.to_uppercase();
        }

        word
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

impl ToScheme for BasicConfig {
    fn to_scheme(&self) -> Scheme {
        Scheme {
            word_generator: Box::new(FixedGenerator::new(self.words)),
            word_processors: vec![
                Box::new(WordCapitalizer::new(self.capitalize_first, self.capitalize_words)),
            ],
            phrase_builder: Box::new(BasicPhraseBuilder::new(self.separator.clone())),
            phrase_processors: Vec::new(),
        }
    }
}

// TODO: find a better abstract chances type for this.
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

impl Entropy for Occurrence {
    fn entropy(&self) -> f64 {
        match self {
            Occurrence::Sometimes => 2.0,
            _ => 1.0,
        }
    }
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
