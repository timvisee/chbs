//! Generation scheme module to define how to generate passphrases
//!
//! This module defines the [`Scheme`](Scheme) type, with a corresponding [build](SchemeBuilder) if
//! that pattern is desired.
//!
//! As both provided and custom structures may produce a [`Scheme`](Scheme) for passphrase
//! generation, the [`ToScheme`](ToScheme) trait is used for a generic way of doing this.

use crate::entropy::Entropy;
use crate::prelude::*;

/// A passphrase generation scheme.
///
/// The scheme defines how passphrases should be generated, and can be directly used to so.
/// This scheme holds various components used during generation to modify and combine passphrase
/// parts or words. The scheme may be used as iterator, which will produce an infinite number of
/// passphrases.
///
/// It is recommended to use a configuration struct to confige and build a specific `Scheme`
/// instead of setting one up manually.
/// The `chbs` crate provides [`BasicConfig`](::config::BasicConfig).
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
/// - A set of word stylers is used to modify each passphrase word from the generated set, to
///   randomize capitalization, to add special characters and more depending on their
///   configuration. Each styler is applied once to each phrase word in the specified order.
///   If no word styler is available, the words are kept intact.
/// - The phrase builder combines the set of now modified passphrase words into a full passphrase,
///   the builder separates the words with a space or anything else depending on it's
///   configuration.
/// - A set of phrase stylers is used to modify the full passphrase that is now combined. They
///   may be used for further modifications with full control over the phrase. If no phrase
///   styler is available, the phrase is kept intact.
///
/// # Examples
///
/// The scheme implements `Iterator`. You may easily generate many passphrases this way:
///
/// ```rust
/// use chbs::{config::BasicConfig, prelude::*, scheme::Scheme};
///
/// let scheme = BasicConfig::default().to_scheme();
///
/// scheme.take(8)
///     .for_each(|passphrase| println!("{}", passphrase));
/// ```
#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct Scheme {
    /// A word set provider, which sources a set of random words to use in the passphrase.
    word_set_provider: Box<dyn WordSetProvider>,

    /// A set of word stylers to apply to each passphrase word.
    word_stylers: Vec<Box<dyn WordStyler>>,

    /// A phrase builder that builds a passphrase out of a styled set of passphrase words.
    phrase_builder: Box<dyn PhraseBuilder>,

    /// A set of phrase stylers to apply to each passphrase.
    phrase_stylers: Vec<Box<dyn PhraseStyler>>,
}

impl Scheme {
    /// Construct a scheme with the given components
    ///
    /// When all components for a scheme are collected, a scheme can be constructed using this
    /// method.
    ///
    /// If you prefer the builder pattern to build a scheme, use [`SchemeBuilder`](SchemeBuilder)
    /// instead.
    pub fn new(
        word_set_provider: Box<dyn WordSetProvider>,
        word_stylers: Vec<Box<dyn WordStyler>>,
        phrase_builder: Box<dyn PhraseBuilder>,
        phrase_stylers: Vec<Box<dyn PhraseStyler>>,
    ) -> Self {
        Self {
            word_set_provider,
            word_stylers,
            phrase_builder,
            phrase_stylers,
        }
    }

    /// Build a configuration based on the given object.
    pub fn from<S: ToScheme>(config: &S) -> Self {
        config.to_scheme()
    }

    /// Construct a [`SchemeBuilder`](SchemeBuilder) for building a [`Scheme`](Scheme)
    pub fn build() -> SchemeBuilder {
        SchemeBuilder::default()
    }

    /// Generate a single passphrase based on this scheme.
    pub fn generate(&self) -> String {
        // Generate the passphrase words
        let mut words = self.word_set_provider.words();

        // Run the passphrase words through the word stylers
        for p in &self.word_stylers {
            words = words.into_iter().map(|w| p.style_word(w)).collect();
        }

        // Build the passphrase
        let mut phrase = self.phrase_builder.build_phrase(words);

        // Run the phrase through the passphrase stylers
        for p in &self.phrase_stylers {
            phrase = p.style_phrase(phrase);
        }

        phrase
    }

    /// Calculate the entropy that passphrases based on this scheme have.
    ///
    /// See the documentation on [Entropy](Entropy) for details on what entropy is and how it
    /// should be calculated.
    pub fn entropy(&self) -> Entropy {
        self.word_set_provider.entropy()
            + self
                .word_stylers
                .iter()
                .map(|p| p.entropy())
                .sum::<Entropy>()
            + self.phrase_builder.entropy()
            + self
                .phrase_stylers
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

impl SchemeBuilder {
    /// Add a single word styler to the scheme.
    pub fn add_word_styler(mut self, styler: Box<dyn WordStyler>) -> Self {
        match self.word_stylers {
            Some(ref mut stylers) => stylers.push(styler),
            None => self.word_stylers = Some(vec![styler]),
        }

        self
    }

    /// Add a single phrase styler to the scheme.
    pub fn add_phrase_styler(mut self, styler: Box<dyn PhraseStyler>) -> Self {
        match self.phrase_stylers {
            Some(ref mut stylers) => stylers.push(styler),
            None => self.phrase_stylers = Some(vec![styler]),
        }

        self
    }
}

/// A trait providing an interface to build a password scheme based on some sort of configuration.
pub trait ToScheme {
    /// Build a password scheme based on configuration in this object.
    fn to_scheme(&self) -> Scheme;
}
