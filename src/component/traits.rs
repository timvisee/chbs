//! Component kind traits
//!
//! The function of this trait is defined in the [`component`](super) module.

use std::fmt::Debug;

use crate::prelude::*;

/// Something that provides random words.
///
/// A word provider is used to provide any number of words for passphrase generation.
/// Whether random words are genrated, or whether they are sampled from a known wordlist is
/// undefined and decided by the implementor. Providers must be infinite and should never deplete.
/// It is possible that the same word may be obtained more than once.
///
/// When generating a passphrase a set of words is obtained from a word provider by subsequent
/// calls to [`word`](WordProvider::word).
///
/// This trait is not used as component kind on [`Scheme`](::scheme::Scheme), it may however be
/// useful to implement on types that support this functionallity. In addition to that, the
/// [`WordSetProvider`](WordSetProvider) should be easy to implement on types that implement this
/// trait.
pub trait WordProvider: HasEntropy + Debug + Clone + IntoIterator<Item = String> + Send + Sync {
    /// Obtain a random word.
    ///
    /// This method should obtain and return a random word from the provider.
    /// The randomization must be cryptographically secure as it's used for generating passphrases.
    fn word(&self) -> String;
}

/// Something that provides sets of random words.
///
/// A component that provides functionallity to source a random set of passphrase words.
/// On sourcing, an ordered list of random passphrase words is returned that will be used in the
/// password.
///
/// This differs from [`WordProvider`](WordProvider) as this provides a set of words instead of a
/// single word. It should be fairly easy to implement this trait on types that have the
/// [`WordProvider`](WordProvider) implemented.
pub trait WordSetProvider: HasEntropy + Debug + Send + Sync {
    /// Source a set of random passphrase words to use in a passphrase.
    fn words(&self) -> Vec<String>;
}

/// Something that provides logic to _style_ each passphrase word.
/// This could be used to build a styler for word capitalization.
pub trait WordStyler: HasEntropy + Debug + Send + Sync {
    /// Style the given `word`.
    fn style_word(&self, word: String) -> String;
}

/// Something that provides logic to combine a list of passphrase words into a passphrase.
pub trait PhraseBuilder: HasEntropy + Debug + Send + Sync {
    /// Build the passphrase from the given words, and combine them in one final passphrase.
    fn build_phrase(&self, words: Vec<String>) -> String;
}

/// Something that provides logic to _style_ a passphrase as a whole.
pub trait PhraseStyler: HasEntropy + Debug + Send + Sync {
    /// Style the given `phrase` as a whole.
    /// The styled passphrase is returned.
    fn style_phrase(&self, phrase: String) -> String;
}
