//! Passphrase word related components
//!
//! This module provides some component implementations for processing words.
//! These components implement any of the following component kind traits:
//!
//! - [`WordSetProvider`](super::traits::WordSetProvider)
//! - [`WordStyler`](super::traits::WordStyler)
//!
//! Most of these components are used by configuration strucutres provided by this crate, see
//! the [`config`](::config) module. You may of course implement these components in your own
//! configuration structures and [`Scheme`](::scheme::Scheme) definitions.

use rand::thread_rng;

use crate::entropy::Entropy;
use crate::prelude::*;
use crate::probability::Probability;

/// A generator providing a fixed number of passphrase words.
///
/// This generator provides a set of passphrase words for passphrase generation with a fixed number
/// of words based on the configuration.
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
    /// 7776 (6<sup>5</sup>) words.
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
    fn words(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for _ in 0..self.words {
            res.push(self.provider.word());
        }
        res
    }
}

/// A word styler to capitalize passphrase words.
///
/// This word styler component capitalizes words for a passphrase in different styles depending
/// on it's configuration. This styler currently supports capitalization of the first character
/// in words and/or passphrase words as a whole.
#[derive(Debug)]
pub struct WordCapitalizer {
    /// Whether to capitalize the first characters of words.
    first: Probability,

    /// Whether to capitalize whole words.
    all: Probability,
}

impl WordCapitalizer {
    /// Construct the word capitalizer component
    ///
    /// Whehter to capitalize the first character or the whole word must be defined using the
    /// `first` and `all` parameters.
    pub fn new(first: Probability, all: Probability) -> Self {
        Self { first, all }
    }
}

impl HasEntropy for WordCapitalizer {
    fn entropy(&self) -> Entropy {
        // For capitalizing all, capitalizing the first character doesn't change anything
        if let Probability::Always = self.all {
            Entropy::zero()
        } else {
            self.first.entropy() + self.all.entropy()
        }
    }
}

impl WordStyler for WordCapitalizer {
    fn style_word(&self, mut word: String) -> String {
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
