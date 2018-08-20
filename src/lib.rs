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

// TODO: create wordlist struct
// TODO: use wordlist in word sampler
// TODO: create word generator using sampler or random generator

#[macro_use]
extern crate derive_builder;
extern crate rand;

use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    string::ToString,
};

use rand::{distributions::Uniform, prelude::*, rngs::ThreadRng, thread_rng};

/// A static wordlist to use.
const WORDLIST: &str = include_str!("../res/eff_large_wordlist.txt");

/// The default number of words the passphrase will consist of.
const DEFAULT_WORDS: usize = 5;

/// The default separator used between passphrase words.
const DEFAULT_SEPARATOR: &str = " ";

/// Build a vector of words based on a wordlist to use for passphrase
/// generation.
///
/// The included wordlist is prefixed with dice numbers, each line ends with
/// an actual word. This function grabs the last alphabetic word from each
/// line which are then collected into a vector.
pub fn words<'a>() -> Vec<&'a str> {
    WORDLIST
        .lines()
        .filter_map(|line| {
            line.trim_right()
                .rsplit_terminator(char::is_whitespace)
                .next()
        }).collect()
}

/// Build a word sampler which is an iterator that randomly samples words from
/// the included wordlist.
///
/// The word sampler is concidered cryptographically secure.
pub fn word_sampler() -> WordSampler {
    WordSampler::new(
        words().into_iter()
            .map(|s| s.to_owned())
            .collect()
    )
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

/// An iterator providing sampled words.
///
/// This sampler uses a given wordlist of wich random words are picked for use in passphrases.
/// The randomization is concidered cryptographically secure.
///
/// The iterator is infinite, as much words as needed may be pulled from this iterator.
///
/// To construct a `WordSampler` from a [`WordList`](WordList) use [`sampler`](WordList::sampler).
// TODO: use string references
#[derive(Clone, Debug)]
pub struct WordSampler {
    /// List of words that is used for sampling.
    words: Vec<String>,

    /// Random distribution used for sampling.
    distribution: Uniform<usize>,

    /// Random number generator used for sampling.
    rng: ThreadRng,
}

impl WordSampler {
    /// Build a new word sampler which samples the given word list.
    pub fn new(words: Vec<String>) -> WordSampler {
        WordSampler {
            distribution: Uniform::new(0, words.len()),
            words,
            rng: thread_rng(),
        }
    }

    /// Sample a random word by reference.
    ///
    /// This returns a cryptographically secure random word by reference, which is faster than
    /// [`word`](WordSampler::word) as it prevents cloning the chosen word.
    fn word_ref(&mut self) -> &str {
        // TODO: maybe use rng.choose
        &self.words[self.rng.sample(self.distribution)]
    }
}

impl WordProvider for WordSampler {
    fn word(&mut self) -> String {
        self.word_ref().to_owned()
    }
}

impl HasEntropy for WordSampler {
    fn entropy(&self) -> Entropy {
        Entropy::from_real(self.words.len() as f64)
    }
}

impl Iterator for WordSampler {
    type Item = String;

    /// Sample the next random word.
    /// This iterator is infinite and always returns some word.
    fn next(&mut self) -> Option<String> {
        Some(self.word())
    }
}

/// A passphrase generation scheme.
///
/// The scheme defines how passphrases should be generated, and can be directly used to so.
/// This scheme holds various components used during generation to modify and combine passphrase
/// parts or words. The scheme may be used as iterator, which will produce an infinite number of
/// passphrases.
///
/// It is recommended to use a configuration struct to confige and build a specific `Scheme`
/// instead of setting one up manually. The `chbs` crate provides [`BasicConfig`](BasicConfig).
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
///
/// # Examples
///
/// The scheme implements [`Iterator`](std::iter::Iterator). You may easily generate many
/// passphrases this way:
///
/// ```rust
/// let scheme = Scheme::default();
///
/// scheme.take(8)
///     .for_each(|passphrase|
///         println!("{}", passphrase);
///     );
/// ```
#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct Scheme {
    /// A word set provider, which sources a set of random words to use in the passphrase.
    word_set_provider: Box<dyn WordSetProvider>,

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
        word_set_provider: Box<dyn WordSetProvider>,
        word_processors: Vec<Box<dyn WordProcessor>>,
        phrase_builder: Box<dyn PhraseBuilder>,
        phrase_processors: Vec<Box<dyn PhraseProcessor>>,
    ) -> Self {
        Self {
            word_set_provider,
            word_processors,
            phrase_builder,
            phrase_processors,
        }
    }

    /// Build a configuration based on the given object.
    pub fn from<S: ToScheme>(config: &S) -> Self {
        config.to_scheme()
    }

    /// Generate a single passphrase based on this scheme.
    fn generate(&mut self) -> String {
        // Generate the passphrase words
        let mut words = self.word_set_provider.words();

        // Run the passphrase words through the word processors
        for p in &self.word_processors {
            words = words.into_iter().map(|w| p.process_word(w)).collect();
        }

        // Build the passphrase
        let mut phrase = self.phrase_builder.build_phrase(words);

        // Run the phrase through the passphrase processors
        for p in &self.phrase_processors {
            phrase = p.process_phrase(phrase);
        }

        phrase
    }

    /// Calculate the entropy passphrases based on this scheme have.
    ///
    /// See the documentation on [Entropy](Entropy) for details on what entropy is and how it
    /// should be calculated.
    pub fn entropy(&self) -> Entropy {
        self.word_set_provider.entropy()
            + self
                .word_processors
                .iter()
                .map(|p| p.entropy())
                .sum::<Entropy>()
            + self.phrase_builder.entropy()
            + self
                .phrase_processors
                .iter()
                .map(|p| p.entropy())
                .sum::<Entropy>()
    }
}

impl Iterator for Scheme {
    type Item = String;

    /// Generate a new passphrase based on this scheme.
    ///
    /// This method always returns `Some` holding a passphrase.
    fn next(&mut self) -> Option<String> {
        Some(self.generate())
    }
}

/// A trait providing an interface to build a password scheme based on some sort of configuration.
pub trait ToScheme {
    /// Build a password scheme based on configuration in this object.
    fn to_scheme(&self) -> Scheme;
}

/// Password entropy.
///
/// The entropy number used internally represents the number of base 2 entropy bits,
/// and is calculated using `log2(choices)`.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Entropy(f64);

impl Entropy {
    /// Construct entropy for zero bits, representing no entropy.
    pub fn zero() -> Self {
        Entropy(0.0)
    }

    /// Construct entropy for one bit, representing 50/50 chance.
    pub fn one() -> Self {
        Entropy(1.0)
    }

    /// Construct entropy from a number of entropy bits.
    pub fn from_bits<F: Into<f64>>(bits: F) -> Self {
        Entropy(bits.into())
    }

    /// Construct entropy from a real number.
    ///
    /// For a wordlist of 7776 words where choices are uniform, the real number `7776` may be given
    /// to construct the proper entropy value. This would produce an entropy instance with about
    /// `12.9` bits.
    ///
    /// If `Entropy` should be constructed from a number of bits, use
    /// [`from_bits`](Entropy::from_bits) instead.
    pub fn from_real<F: Into<f64>>(real: F) -> Self {
        Entropy(real.into().log2())
    }

    /// Get the number of entropy bits.
    pub fn bits(&self) -> f64 {
        self.0
    }
}

impl Sum for Entropy {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator,
        I::Item: Into<Entropy>,
    {
        iter.fold(Entropy::zero(), |total, e| total + e.into())
    }
}

/// A macro to derive common operator traits for a struct.
macro_rules! derive_ops {
    (impl $trait_: ident for $type_: ident { fn $method: ident }) => {
        // Operation with another Entropy object
        impl $trait_<$type_> for $type_ {
            type Output = $type_;

            fn $method(self, $type_(b): $type_) -> $type_ {
                let $type_(a) = self;
                $type_(a.$method(&b))
            }
        }

        // Operation with another integer or float value
        impl<B> $trait_<B> for $type_
        where
            B: Into<f64>,
        {
            type Output = $type_;

            fn $method(self, b: B) -> $type_ {
                let $type_(a) = self;
                $type_(a.$method(&b.into()))
            }
        }
    };
}

derive_ops! { impl Add for Entropy { fn add } }
derive_ops! { impl Sub for Entropy { fn sub } }
derive_ops! { impl Mul for Entropy { fn mul } }
derive_ops! { impl Div for Entropy { fn div } }

/// An entropy source.
///
/// Get the entropy value for the current component, whether that is a word processor, a phrase
/// builder or something else.
///
/// TODO: properly describe what entropy is here.
pub trait HasEntropy {
    /// Get the entropy value for this whole component.
    /// The returned entropy value may be accumulated from various internal entropy sources.
    ///
    /// See the documentation on [Entropy](Entropy) for details on what entropy is and how it
    /// should be calculated.
    /// If this component does not have any effect on passphrase entropy `1` should be returned.
    /// TODO: use an entropy struct to track the entropy value
    fn entropy(&self) -> Entropy;
}

/// A wordlist.
///
/// A loaded fixed wordlist which may be used as word provider for passphrase generation by
/// constructing a sampler using [`sampler`](WordList::sampler).
#[derive(Clone, Debug)]
pub struct WordList {
    /// A fixed set of words.
    words: Vec<String>,
}

impl WordList {
    /// Construct a new word list with the given words.
    /// TODO: panic if the list contains no words
    pub fn new(words: Vec<String>) -> Self {
        WordList { words }
    }

    // TODO: load a wordlist from a file
    // TODO: load statically included wordlists

    /// Construct a word sampler based on this wordlist.
    ///
    /// The word sampler may be used to pull any number of random words from the wordlist for
    /// passphrase generation.
    pub fn sampler(&self) -> WordSampler {
        WordSampler::new(self.words.clone())
    }
}

impl Default for WordList {
    fn default() -> WordList {
        WordList::new(
            words().into_iter()
                .map(|s| s.to_owned())
                .collect()
        )
    }
}

/// Something that provides random words.
///
/// A word provider is used to provide any number of words for passphrase generation.
/// Whether random words are genrated, or whether they are sampled from a known wordlist is
/// undefined and decided by the implementor. Providers must be infinite and should never deplete.
/// It is possible that the same word may be obtained more than once.
///
/// When generating a passphrase a set of words is obtained from a word provider by subsequent
/// calls to [`word`](WordProvider::word).
pub trait WordProvider: HasEntropy + Debug + Iterator<Item = String> + Clone {
    /// Obtain a random word.
    ///
    /// This method should obtain and return a random word from the provider.
    /// The randomization must be cryptographically secure as it's used for generating passphrases.
    fn word(&mut self) -> String;
}

/// A component that provides functionallity to source a random set of passphrase words.
/// On sourcing, an ordered list of random passphrase words is returned that will be used in the
/// password.
pub trait WordSetProvider: HasEntropy + Debug {
    /// Source a set of random passphrase words to use in a passphrase.
    fn words(&mut self) -> Vec<String>;
}

/// Something that provides logic to process each passphrase word.
/// This could be used to build a processor for word capitalization.
pub trait WordProcessor: HasEntropy + Debug {
    /// Process the given `word`.
    fn process_word(&self, word: String) -> String;
}

/// Something that provides logic to combine a list of passphrase words into a passphrase.
pub trait PhraseBuilder: HasEntropy + Debug {
    /// Build the passphrase from the given words, and combine them in one final passphrase.
    fn build_phrase(&self, words: Vec<String>) -> String;
}

/// Something that provides logic to process a passphrase as a whole.
pub trait PhraseProcessor: HasEntropy + Debug {
    /// Process the given `phrase` as a whole.
    /// The processed passphrase is returned.
    fn process_phrase(&self, phrase: String) -> String;
}

/// A generator providing a fixed number of passphrase words.
///
/// This generator provides a set of passphrase words for passphrase generation with a fixed number
/// of words based on the configuration.
///
/// TODO: configure the wordlist to use.
#[derive(Debug)]
pub struct FixedWordSetProvider<P>
where
    P: WordProvider,
{
    /// The word provider to obtain words from.
    provider: P,

    /// The number of passphrase words to obtain.
    words: usize,
}

impl<P> FixedWordSetProvider<P>
where
    P: WordProvider,
{
    /// Construct a word set provider with a fixed word count.
    ///
    /// The number of words to fill a set with must be provided as `words`.
    /// It is recommended to use at least 5 passphrase words with a wordlist of at least
    /// 7776 (6^5) words.
    ///
    /// # Panic
    ///
    /// `words` must be higher than zero.
    pub fn new(provider: P, words: usize) -> Self {
        // At least 1 word must be obtained by this set provider
        if words == 0 {
            panic!("cannot construct FixedWordSetProvider that obtains zero words");
        }

        Self { provider, words }
    }
}

impl<P> HasEntropy for FixedWordSetProvider<P>
where
    P: WordProvider,
{
    fn entropy(&self) -> Entropy {
        self.provider.entropy() * self.words as f64
    }
}

impl<P> WordSetProvider for FixedWordSetProvider<P>
where
    P: WordProvider,
{
    fn words(&mut self) -> Vec<String> {
        self.provider
            .by_ref()
            .take(self.words)
            .map(|word| word.to_owned())
            .collect()
    }
}

/// A word processor to capitalize passphrase words.
///
/// This word processor component capitalizes words for a passphrase in different styles depending
/// on it's configuration. This processor currently supports capitalization of the first character
/// in words and/or passphrase words as a whole.
#[derive(Debug)]
pub struct WordCapitalizer {
    /// Whether to capitalize the first characters of words.
    first: Probability,

    /// Whether to capitalize whole words.
    all: Probability,
}

impl WordCapitalizer {
    pub fn new(first: Probability, all: Probability) -> Self {
        Self { first, all }
    }
}

impl HasEntropy for WordCapitalizer {
    fn entropy(&self) -> Entropy {
        self.first.entropy() + self.all.entropy()
    }
}

impl WordProcessor for WordCapitalizer {
    fn process_word(&self, mut word: String) -> String {
        if word.is_empty() {
            return word;
        }

        let mut rng = thread_rng();

        // Capitalize the first character
        if self.first.gen_bool(&mut rng) {
            let first = word
                .chars()
                .map(|c| c.to_uppercase().to_string())
                .next()
                .unwrap_or_else(String::new);
            let rest: String = word.chars().skip(1).collect();
            word = first + &rest;
        }

        // Capitalize whole words
        if self.all.gen_bool(&mut rng) {
            word = word.to_uppercase();
        }

        word
    }
}

/// A passphrase builder with as constant word separator.
///
/// This is a basic passphrase builder that uses a given set of words to build a full passphrase.
/// This builder uses a single fixed separator, that is used as glue between all the passphrase
/// words.
#[derive(Debug)]
pub struct BasicPhraseBuilder {
    /// The separator that is used.
    separator: String,
}

impl BasicPhraseBuilder {
    pub fn new(separator: String) -> Self {
        Self { separator }
    }
}

impl HasEntropy for BasicPhraseBuilder {
    fn entropy(&self) -> Entropy {
        Entropy::zero()
    }
}

impl PhraseBuilder for BasicPhraseBuilder {
    fn build_phrase(&self, words: Vec<String>) -> String {
        words.join(&self.separator)
    }
}

/// A simple passphrase configuration struct.
///
/// This struct provides basic passphrase generation options for simple passphrases.
/// When the struct is configured, a [`Scheme`](Scheme) may be created based on it to actually
/// generate passphrases.
///
/// # Examples
///
/// Use the default basic configuration, and change the separator. Then build a scheme, and
/// generate a passphrase.
///
/// ```rust
/// // Define the configuration
/// let mut config = BasicConfig::default();
/// config.separator = "-".into();
///
/// // Build the scheme for generation
/// let scheme = config.to_scheme();
///
/// // Generate and output
/// println!("Passphrase: {}", scheme.generate());
/// ```
///
/// Or use the [`BasicConfigBuilder`](BasicConfigBuilder) instead for a builder pattern:
///
/// ```rust
/// let config = BasicConfigBuilder::default()
///     .separator("-")
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(setter(into))]
pub struct BasicConfig<P>
where
    P: WordProvider,
{
    /// The number of words the passphrase will consist of.
    pub words: usize,

    /// A provider random passphrase words can be obtained from.
    pub word_provider: P,

    /// The separator string to use between passphrase words.
    pub separator: String,

    /// Whether to capitalize the first characters of words.
    pub capitalize_first: Probability,

    /// Whether to capitalize whole words.
    pub capitalize_words: Probability,
}

impl Default for BasicConfig<WordSampler> {
    fn default() -> BasicConfig<WordSampler> {
        BasicConfig {
            words: DEFAULT_WORDS,
            word_provider: WordList::default().sampler(),
            separator: DEFAULT_SEPARATOR.into(),
            capitalize_first: Probability::half(),
            capitalize_words: Probability::Never,
        }
    }
}

impl<P> ToScheme for BasicConfig<P>
where
    P: WordProvider + 'static,
{
    fn to_scheme(&self) -> Scheme {
        SchemeBuilder::default()
            .word_set_provider(
                Box::new(FixedWordSetProvider::new(
                    self.word_provider.clone(),
                    self.words
                ))
            )
            .word_processors(vec![
                Box::new(WordCapitalizer::new(
                    self.capitalize_first,
                    self.capitalize_words,
                ))
            ])
            .phrase_builder(Box::new(BasicPhraseBuilder::new(self.separator.clone())))
            .phrase_processors(Vec::new())
            .build()
            .unwrap()
    }
}

/// A probability definition.
///
/// This defines what the probability is of something being true.
///
/// The function [`gen_bool`](Probability::gen_bool) can be used to generate a boolean based on
/// this probability. Depending on what randomness source is given, it may be cryptographically
/// secure.
#[derive(Copy, Clone, Debug)]
pub enum Probability {
    /// This is always true.
    Always,

    /// This is sometimes true.
    ///
    /// If `1.0` it's always true, if `0.0` it is never true, the value may be anywhere in between.
    ///
    /// If the value is exactly `0.0` or `1.0` the variants [`Always`](Probability::Always) and
    /// [`Never`](Probability::Never) should be used instead.
    /// It is therefore recommended to construct this type using the
    /// [`from`](Probability::from) method as this automatically chooses the correct variant.
    ///
    /// This value may never be `p < 0` or `p > 1`, as it will cause panics.
    Sometimes(f64),

    /// This is never true, and is always false.
    Never,
}

impl Probability {
    /// Construct a probability from the given probability value.
    ///
    /// If `1.0` it's always true, if `0.0` it is never true.
    /// Values outside this range will be wrapped to their corresponding edge.
    pub fn from(probability: f64) -> Self {
        match probability {
            x if x >= 1.0 => Probability::Always,
            x if x <= 0.0 => Probability::Never,
            x => Probability::Sometimes(x),
        }
    }

    /// Construct a probability from the given percentage.
    ///
    /// If `100.0` it's always true, if `0.0` it is never true.
    /// Values outside this range will be wrapped to their corresponding edge.
    pub fn from_percentage(percentage: f64) -> Self {
        match percentage {
            x if x >= 100.0 => Probability::Always,
            x if x <= 0.0 => Probability::Never,
            x => Probability::Sometimes(x / 100.0),
        }
    }

    /// Construct a probability that is true half of the times (50/50, 50%).
    pub fn half() -> Self {
        Self::from(0.5)
    }

    /// Get the probability value.
    ///
    /// To get the percentage, use [`percentage`](Probability::percentage).
    pub fn value(&self) -> f64 {
        match self {
            Probability::Always => 1.0,
            Probability::Sometimes(p) => *p,
            Probability::Never => 0.0,
        }
    }

    /// Get the probability percentage.
    ///
    /// To get the probability value, use [`value`](Probability::value).
    pub fn percentage(&self) -> f64 {
        self.value() * 100.0
    }

    /// Generate a boolean for this probability.
    ///
    /// If the given randomness source to `rng` is cryptographically secure,
    /// the generated boolean can be considered cryptographically secure as well.
    pub fn gen_bool<R: Rng>(self, rng: &mut R) -> bool {
        match self {
            Probability::Always => true,
            Probability::Never => false,
            Probability::Sometimes(p) => rng.gen_bool(p),
        }
    }

    /// Generate a cryptographically secure boolean for this probability.
    ///
    /// This method obtains a cryptographically secure randomness source through
    /// [`thread_rng`](rand::thread_rng) and generates a boolean through
    /// [`gen_bool`](Probability::gen_bool).
    pub fn gen_bool_secure(self) -> bool {
        match self {
            Probability::Always => true,
            Probability::Never => false,
            Probability::Sometimes(_) => self.gen_bool(&mut thread_rng()),
        }
    }
}

impl HasEntropy for Probability {
    fn entropy(&self) -> Entropy {
        match self {
            // TODO: properly calculate entropy here
            Probability::Sometimes(_p) => Entropy::one(),
            _ => Entropy::zero(),
        }
    }
}

/// Allow easy `Probability` selection of `Always` and `Never` from a boolean.
impl From<bool> for Probability {
    fn from(b: bool) -> Probability {
        if b {
            Probability::Always
        } else {
            Probability::Never
        }
    }
}

#[cfg(test)]
mod tests {
    use {config, passphrase, word_sampler, words, WordSampler};

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
            words()
                .iter()
                .all(|word| word.chars().all(|c| c.is_alphabetic() || c == '-'))
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
        assert_eq!(word_sampler().take(ITERS).count(), ITERS,);
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
