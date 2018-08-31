use std::fmt::Debug;

use prelude::*;

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

/// Something that provides sets of random words.
///
/// A component that provides functionallity to source a random set of passphrase words.
/// On sourcing, an ordered list of random passphrase words is returned that will be used in the
/// password.
///
/// This differs from [`WordProvider`](WordProvider) as this provides a set of words instead of a
/// single word.
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
