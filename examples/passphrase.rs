//! An example on how to generate a passphrase with a specified number of
//! words.
//!
//! Note that this example prints the generated passphrase in the console,
//! which might not be desired in most situations.

extern crate chbs;
use chbs::passphrase;

fn main() {
    println!("Passphrase: {:?}", passphrase(5));
}
