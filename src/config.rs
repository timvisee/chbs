//! Provided structures to easily configure passphrase generation [schemes](::scheme::Scheme)
//!
//! This module contains various configuration structures that allow easy configuring of a
//! [`Scheme`](::scheme::Scheme) used to define how passphrases are generated.
//!
//! These predefined configuration structures use a predefined set of [components](::component) and
//! support configurability through struct fields. All configuration strucutres have a matching
//! builder to use if you prefer to use the builder pattern.
//!
//! The most basic configuration structure provides is [`BasicConfig`](BasicConfig), see it's
//! documentation for information on how to use it and for some examples.

use crate::component::{
    phrase::BasicPhraseBuilder,
    word::{FixedWordSetProvider, WordCapitalizer},
};
use crate::prelude::*;
use crate::probability::Probability;
use crate::scheme::{Scheme, SchemeBuilder};
use crate::word::{WordList, WordSampler};

use super::{DEFAULT_SEPARATOR, DEFAULT_WORDS};

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
/// use chbs::{config::BasicConfig, prelude::*};
///
/// // Define the configuration
/// let mut config = BasicConfig::default();
/// config.separator = "-".into();
///
/// // Build the scheme for generation
/// let mut scheme = config.to_scheme();
///
/// // Generate and output
/// println!("Passphrase: {}", scheme.generate());
/// ```
///
/// Or use the [`BasicConfigBuilder`](BasicConfigBuilder) instead for a builder pattern:
///
/// ```rust
/// // TODO: fix this example
/// // extern crate chbs;
/// // use chbs::{config::*, word::WordSampler};
/// //
/// // let config = BasicConfigBuilder::default()
/// //     .separator("-")
/// //     .build()
/// //     .unwrap();
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
    /// Build a default basic configuration instance.
    ///
    /// This configuration uses the defaul wordlist as word provider for generating passphrases.
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
            .word_set_provider(Box::new(FixedWordSetProvider::new(
                self.word_provider.clone(),
                self.words,
            )))
            .word_stylers(vec![Box::new(WordCapitalizer::new(
                self.capitalize_first,
                self.capitalize_words,
            ))])
            .phrase_builder(Box::new(BasicPhraseBuilder::new(self.separator.clone())))
            .phrase_stylers(Vec::new())
            .build()
            .unwrap()
    }
}
