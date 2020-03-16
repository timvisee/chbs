//! Passphrase related components
//!
//! This module provides some component implementations for processing passphrases.
//! These components implement any of the following component kind traits:
//!
//! - [`PhraseBuilder`](super::traits::PhraseBuilder)
//! - [`PhraseStyler`](super::traits::PhraseStyler)
//!
//! Most of these components are used by configuration strucutres provided by this crate, see
//! the [`config`](::config) module. You may of course implement these components in your own
//! configuration structures and [`Scheme`](::scheme::Scheme) definitions.

use crate::entropy::Entropy;
use crate::prelude::*;

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
