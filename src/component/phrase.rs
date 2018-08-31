use entropy::Entropy;
use prelude::*;

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
