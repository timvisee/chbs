//! An example on how to use the word sampler to generate random words based
//! on the included wordlist.

extern crate chbs;
use chbs::word_sampler;

fn main() {
    let mut sampler = word_sampler();

    for i in 0..8 {
        println!(
            "Sampled word #{}: {:?}",
            i,
            sampler.next().unwrap(),
        );
    }
}
