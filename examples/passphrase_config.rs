//! An example on how to generate a passphrase with a specified number of
//! words.
//!
//! Note that this example prints the generated passphrase in the console,
//! which might not be desired in most situations.

extern crate chbs;
use chbs::*;

fn main() {
    // Build a custom configuration to:
    // - use hyphens as separator
    // - sometimes capitalizes whole passphrase words
    // - sometimes capitalizes the first characters of passphrase words
    let config = BasicConfigBuilder::default()
        .separator("-")
        .capitalize_first(Occurrence::Sometimes)
        .capitalize_words(Occurrence::Sometimes)
        .build()
        .unwrap();

    println!("Passphrase: {:?}", passphrase(&config));
}
