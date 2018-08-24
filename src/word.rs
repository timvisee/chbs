use rand::{distributions::Uniform, prelude::*, rngs::ThreadRng, thread_rng};

use entropy::Entropy;
use prelude::*;

use super::words;

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

/// A wordlist.
///
/// A loaded fixed wordlist which may be used as word provider for passphrase generation by
/// constructing a sampler using [`sampler`](WordList::sampler).
///
/// It is highly recommended that the worlist contains at least 7776 (6^5) words to provide enough
/// entropy when uniformly sampling words from it.
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
        WordList::new(words().into_iter().map(|s| s.to_owned()).collect())
    }
}
