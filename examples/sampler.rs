//! Sample passphrase words from a wordlist
//!
//! A minimal example that shows how a word sampler derived from a default wordlist may be used to
//! uniformly sample a number of words.

extern crate chbs;

use chbs::word::WordList;

fn main() {
    let words = WordList::default();
    let sampler = words.sampler();

    for word in sampler.take(8) {
        println!("Sampled word: {:?}", word);
    }
}
