//! Utilities for collecting and generating words for in a passphrase
//!
//! This module provides various constructs for collecting and/or generating words to use in a
//! passphrase.
//!
//! The [`WordList`](WordList) structure is used for static wordlists, which may be uniformly
//! sampled using a [`WordSampler`](WordSampler).
//!
//! Constants holding a static built-in wordlist are included so providing your own wordlist is not
//! required, see the [`BUILTIN_`](#constants) constants.
//! These lists can easily be loaded using the [`buildin_`](WordList) methods on
//! [`WordList`](WordList).

use std::fs::read_to_string;
use std::path::Path;

use thiserror::Error;
use rand::{distributions::Uniform, prelude::*};

use crate::entropy::Entropy;
use crate::prelude::*;

/// The built-in EFF large wordlist words.
///
/// Construct a [`WordList`](WordList) from this list using
/// [`WordList::builtin_eff_large()`](WordList::builtin_eff_large).
///
/// This wordlist contains 7776 (6<sup>5</sup>) words,
/// and has an entropy of about 12.9 bits when uniformly sampling words from it.
///
/// The list is slightly modified to discard the dice numbers,
/// [source](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).
pub const BUILTIN_EFF_LARGE: &str = include_str!("../res/eff/large.txt");

/// The built-in EFF short wordlist words.
///
/// Construct a [`WordList`](WordList) from this list using
/// [`WordList::builtin_eff_short()`](WordList::builtin_eff_short).
///
/// **Note:** this wordlist is considered short, as it only contains 1296 (6<sup>4</sup>) words.
/// The list has an entropy of about 10.3 bits when uniformly sampling words from it.  
/// It is recommended to use a larger wordlist such as [`BUILTIN_EFF_LARGE`](BUILTIN_EFF_LARGE).
///
/// The list is slightly modified to discard the dice numbers,
/// [source](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).
pub const BUILTIN_EFF_SHORT: &str = include_str!("../res/eff/short.txt");

/// The built-in EFF general short wordlist words.
///
/// Construct a [`WordList`](WordList) from this list using
/// [`WordList::builtin_eff_general_short()`](WordList::builtin_eff_general_short).
///
/// **Note:** this wordlist is considered short, as it only contains 1296 (6<sup>4</sup>) words.
/// The list has an entropy of about 10.3 bits when uniformly sampling words from it.  
/// It is recommended to use a larger wordlist such as [`BUILTIN_EFF_LARGE`](BUILTIN_EFF_LARGE).
///
/// The list is slightly modified to discard the dice numbers,
/// [source](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).
pub const BUILTIN_EFF_GENERAL_SHORT: &str = include_str!("../res/eff/general_short.txt");

/// A wordlist.
///
/// To load a built-in wordlist, checkout the methods on this struct prefixed with `builtin_`.  
/// The default wordlist loaded when using `default()` uses
/// [`builtin_eff_large()`](WordList::builtin_eff_large).
///
/// A loaded fixed wordlist which may be used as word provider for passphrase generation by
/// constructing a sampler using [`sampler`](WordList::sampler).
///
/// It is highly recommended that the worlist contains at least 7776 (6<sup>5</sup>) words to
/// provide enough entropy when uniformly sampling words from it.
#[derive(Clone, Debug)]
pub struct WordList {
    /// A fixed set of words.
    words: Vec<String>,
}

impl WordList {
    /// Construct a new word list with the given words.
    ///
    /// To load a wordlist from a file, use [`load`](WordList::load) instead.  
    /// To load a built-in wordlist, use the methods on this struct prefixed with `builtin_`.
    ///
    /// # Panics
    ///
    /// This panics if the given set of words is empty.
    pub fn new(words: Vec<String>) -> Self {
        if words.is_empty() {
            panic!("cannot construct wordlist, given list of words is empty");
        }

        WordList { words }
    }

    /// Load a wordlist from a file.
    ///
    /// This loads a wordlist from a file at the given path, and constructs a `WordList`.
    ///
    /// - Words are splitted by any whitespace (including newlines),
    ///   see `char::is_whitespace`
    /// - It assumes any non-whitespace character is part of a word
    /// - Whitespaces are omitted the final wordlist
    /// - Emtpy items are omitted
    /// - The file must not include dice numbers
    ///
    /// For wordlists that include dice numbers, the [`load_diced`](WordList::load_diced) method
    /// may be used instead.  
    /// If words are separated in a different manner, manually load each word and use the
    /// [`new`](WordList::new) constructor instead.
    ///
    /// An error is returned if loading the wordlist failed, or if the loaded file didn't contain
    /// any words.
    ///
    /// # File examples
    /// ```txt
    /// abacus abdomen abdominal abide abiding
    /// ```
    /// or
    /// ```txt
    /// abacus
    /// abdomen
    /// ```
    pub fn load<P>(path: P) -> Result<Self, WordListError>
    where
        P: AsRef<Path>,
    {
        // Load all words, error if empty
        let words: Vec<String> = read_to_string(path)?
            .split_terminator(char::is_whitespace)
            .filter(|w| !w.is_empty())
            .map(|w| w.to_owned())
            .collect();
        if words.is_empty() {
            return Err(WordListError::Empty);
        }

        Ok(Self::new(words))
    }

    /// Load a diced wordlist from a file.
    ///
    /// This loads a diced wordlist from a file at the given path, and constructs a `WordList`.
    /// Many diceware wordlists include dice numbers for each word, these should be omitted when
    /// using this crate. This method helps with that.
    ///
    /// - Words are splitted by the newline character (`\n`).
    /// - Only the last word on each line is kept, terminated by any whitespace, see
    ///   `char::is_whitespace`
    /// - It assumes any non-whitespace character is part of a word
    /// - Prefixed words do not have to be dice numbers
    /// - Lines having a single word with no dice number prefix are included
    /// - Emtpy lines are omitted
    ///
    /// For wordlists that do not include dice numbers, the the regular [`load`](WordList::load)
    /// method instead.  
    /// If words are separated in a different manner, manually load each word and use the
    /// [`new`](WordList::new) constructor instead.
    ///
    /// An error is returned if loading the wordlist failed, or if the loaded file didn't contain
    /// any words.
    ///
    /// # File examples
    /// ```txt
    /// 11111 abacus
    /// 11112 abdomen
    /// 11113 abdominal
    /// ```
    /// or
    /// ```txt
    /// #1 (1,1,1,1,2)    abacus
    /// #2 (1,1,1,1,2)    abdomen
    /// ```
    pub fn load_diced<P>(path: P) -> Result<Self, WordListError>
    where
        P: AsRef<Path>,
    {
        // Load all words, error if emtpy
        let words: Vec<String> = read_to_string(path)?
            .lines()
            .filter(|w| !w.is_empty())
            .filter_map(|w| w.rsplit_terminator(char::is_whitespace).next())
            .map(|w| w.to_owned())
            .collect();
        if words.is_empty() {
            return Err(WordListError::Empty);
        }

        Ok(Self::new(words))
    }

    /// Construct wordlist from built-in EFF large.
    ///
    /// Use the built-in EFF large list of words, and construct a wordlist from it.
    /// This is based on [`BUILTIN_EFF_LARGE`](BUILTIN_EFF_LARGE).
    pub fn builtin_eff_large() -> Self {
        Self::new(
            BUILTIN_EFF_LARGE
                .lines()
                .map(|w| w.to_owned())
                .collect::<Vec<String>>(),
        )
    }

    /// Construct wordlist from built-in EFF short.
    ///
    /// Use the built-in EFF short list of words, and construct a wordlist from it.
    /// This is based on [`BUILTIN_EFF_SHORT`](BUILTIN_EFF_SHORT).
    ///
    /// **Note:** this wordlist is considered short, as it only contains 1296 (6<sup>4</sup>)
    /// words.
    /// The list has an entropy of about 10.3 bits when uniformly sampling words from it.  
    /// It is recommended to use a larger wordlist such as
    /// [`builtin_eff_large`](WordList::builtin_eff_large).
    pub fn builtin_eff_short() -> Self {
        Self::new(
            BUILTIN_EFF_SHORT
                .lines()
                .map(|w| w.to_owned())
                .collect::<Vec<String>>(),
        )
    }

    /// Construct wordlist from built-in EFF general short.
    ///
    /// Use the built-in EFF general short list of words, and construct a wordlist from it.
    /// This is based on [`BUILTIN_EFF_GENERAL_SHORT`](BUILTIN_EFF_GENERAL_SHORT).
    ///
    /// **Note:** this wordlist is considered short, as it only contains 1296 (6<sup>4</sup>)
    /// words.
    /// The list has an entropy of about 10.3 bits when uniformly sampling words from it.  
    /// It is recommended to use a larger wordlist such as
    /// [`builtin_eff_large`](WordList::builtin_eff_large).
    pub fn builtin_eff_general_short() -> Self {
        Self::new(
            BUILTIN_EFF_GENERAL_SHORT
                .lines()
                .map(|w| w.to_owned())
                .collect::<Vec<String>>(),
        )
    }

    /// Build a sampler for this wordlist.
    ///
    /// The word sampler may be used to pull any number of random words from the wordlist for
    /// passphrase generation.
    pub fn sampler(&self) -> WordSampler {
        WordSampler::new(self.words.clone())
    }
}

impl Default for WordList {
    /// Construct a default wordlist.
    ///
    /// This uses the built-in EFF large wordlist, which can be constructed with
    /// [`WordList::builtin_eff_large()`](WordList::builtin_eff_large).
    fn default() -> WordList {
        WordList::builtin_eff_large()
    }
}

/// A [`WordList`](WordList) error.
#[derive(Error, Debug)]
pub enum WordListError {
    /// Failed to load a wordlist from a file.
    #[error("failed to load wordlist from file")]
    Load(#[from] std::io::Error),

    /// A loaded wordlist is emtpy, which is not allowed.
    #[error("loaded wordlist did not contain words")]
    Empty,
}

/// An iterator uniformly sampling words.
///
/// This sampler uses a given wordlist of wich random words are picked for use in passphrases.
/// The randomization is concidered cryptographically secure.
///
/// The iterator is infinite, as much words as needed may be pulled from this iterator.
///
/// To construct an instance based on a [`WordList`](WordList), use the
/// [`sampler`](WordList::sampler) method.
// TODO: use string references
#[derive(Clone, Debug)]
pub struct WordSampler {
    /// List of words that is used for sampling.
    words: Vec<String>,

    /// Random distribution used for sampling.
    distribution: Uniform<usize>,
}

impl WordSampler {
    /// Build a new word sampler which samples the given word list.
    pub fn new(words: Vec<String>) -> WordSampler {
        WordSampler {
            distribution: Uniform::new(0, words.len()),
            words,
        }
    }

    /// Sample a random word by reference.
    ///
    /// This returns a cryptographically secure random word by reference, which is faster than
    /// [`word`](WordSampler::word) as it prevents cloning the chosen word.
    fn word_ref(&self) -> &str {
        // Used instead of `rng.choose` for better performance
        &self.words[rand::thread_rng().sample(self.distribution)]
    }
}

impl WordProvider for WordSampler {
    fn word(&self) -> String {
        self.word_ref().to_owned()
    }
}

impl HasEntropy for WordSampler {
    fn entropy(&self) -> Entropy {
        Entropy::from_real(self.words.len() as f64)
    }
}

impl IntoIterator for WordSampler {
    type Item = String;
    type IntoIter = WordSamplerIter;

    fn into_iter(self) -> Self::IntoIter {
        WordSamplerIter { sampler: self }
    }
}

pub struct WordSamplerIter {
    sampler: WordSampler,
}

impl Iterator for WordSamplerIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(self.sampler.word())
    }
}