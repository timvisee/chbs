//! An example on how to use the word sampler to generate random words based
//! on the included wordlist.

extern crate chbs;
use chbs::word_sampler;

fn main() {
    let sampler = word_sampler();

    for word in sampler.take(8) {
        println!("Sampled word: {:?}", word);
    }
}
