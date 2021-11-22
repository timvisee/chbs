//! Zero-configuration passphrase generation example
//!
//! This minimal example shows how the [`passphrase`](passphrase) helper can be used to generate a
//! passphrase with zero-configuration. See the documentation of the [`passphrase`](passphrase)
//! function for more details and recommendations.
//!
//! It is recommended to use a configuration structure for generating passphrases to ensure you
//! meet your project requirements, see the [`passphrase_config`](examples/passphrase_config.rs)
//! example.
//!
//! Note that this example prints the generated passphrase in the console,
//! which might not be desired in most situations.

use chbs::passphrase;

fn main() {
    println!("Passphrase: {:?}", passphrase());
}
