//! Sample passphrase words from a wordlist
//!
//! A minimal example that shows how a word sampler derived from a default wordlist may be used to
//! uniformly sample a number of words.
//!
//! It is not recommended to manually use this logic for forming your own passphrases. The
//! abstractions in [`Scheme`](scheme::Scheme) should be used for that possibly with a custom
//! [`config`](config). It is however possible.

use chbs::word::WordList;

fn main() {
    let words = WordList::default();
    let sampler = words.sampler().into_iter();

    for word in sampler.take(8) {
        println!("Sampled word: {:?}", word);
    }
}
