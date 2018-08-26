use rand::thread_rng;

use entropy::Entropy;
use prelude::*;
use probability::Probability;

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
        // For capitalizing all, capitalizing the first character doesn't change anything
        if let Probability::Always = self.all {
            Entropy::zero()
        } else {
            self.first.entropy() + self.all.entropy()
        }
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
