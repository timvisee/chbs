//! # Correct Horse Battery Staple
//! A crate providing secure passphrase generation based on a wordlist also known as
//! [diceware].
//!
//! [![xkcd-img]][xkcd]
//!
//! The name `chbs` is short for the well known "correct horse battery staple"
//! password which originates from the [XKCD][xkcd] comic shown above.
//!
//! This library uses cryptographically secure randomization, and may be used
//! for generating secret passphrases.
//!
//! Notes:
//! * this crate is still in development, and should thus be used with care
//! * no warranty is provided for the quality of the passwords generated
//!   through this library
//!
//! TODO before stabilization which will require API changes:
//! * use secure strings
//! * allow using custom wordlists
//! * ability to configure various passphrase generation properties:
//!   * random word capitalisation
//!   * add numbers
//!   * add special characters
//!   * different separators
//!   * unique words
//! * calculate entropy
//!
//! ## Examples
//! Here are some basic examples on how to use this crate.
//!
//! Add `chbs` as dependency in your `Cargo.toml` first:
//!
//! ```toml
//! [dependencies]
//! chbs = "0.0.1"
//! ```
//!
//! Generate a passphrase with zero configuration using a helper function applying
//! library defaults ([passphrase.rs](examples/passphrase.rs)):
//!
//! ```rust
//! extern crate chbs;
//! use chbs::passphrase;
//!
//! println!("Passphrase: {:?}", passphrase());
//! ```
//!
//! Run it using `cargo run --example passphrase`.
//!
//! Generating a passphrase with configuration is recommended, here is a basic
//! example ([`passphrase_config.rs`](examples/passphrase_config.rs)):
//!
//! ```rust
//! extern crate chbs;
//! use chbs::{config::BasicConfig, prelude::*, probability::Probability};
//!
//! // Build a custom configuration to:
//! let mut config = BasicConfig::default();
//! config.words = 8;
//! config.separator = "-".into();
//! config.capitalize_first = Probability::from(0.33);
//! config.capitalize_words = Probability::half();
//! let mut scheme = config.to_scheme();
//!
//! println!("Passphrase: {:?}", scheme.generate());
//! println!("Entropy: {:?}", scheme.entropy().bits());
//! ```
//!
//! Run it using `cargo run --example passphrase_config`.
//!
//! Use a word sampler to generate an infinite number of random words based on a wordlist
//! ([sampler.rs](examples/sampler.rs)):
//!
//! ```rust
//! extern crate chbs;
//! use chbs::word::WordList;
//!
//! let words = WordList::default();
//! let sampler = words.sampler();
//!
//! for word in sampler.take(8) {
//!     println!("Sampled word: {:?}", word);
//! }
//! ```
//!
//! Run it using `cargo run --example sampler`.
//!
//! ## License
//! This project is released under the MIT license.
//! Check out the [LICENSE](LICENSE) file for more information.
//!
//! [diceware]: https://en.wikipedia.org/wiki/Diceware
//! [xkcd]: https://xkcd.com/936/
//! [xkcd-img]: https://imgs.xkcd.com/comics/password_strength.png

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;
extern crate rand;

pub mod component;
pub mod config;
pub mod entropy;
pub mod prelude;
pub mod probability;
pub mod scheme;
pub mod word;

/// The default number of words the passphrase will consist of.
const DEFAULT_WORDS: usize = 5;

/// The default separator used between passphrase words.
const DEFAULT_SEPARATOR: &str = " ";

use config::BasicConfig;
use prelude::*;

/// Zero-configuration passphrase generation helper
///
/// A quick way to generate a passphrase with no configuration.  
/// Passphrases are based on the [defaults](::config::BasicConfig::default) of a
/// [`BasicConfig`](::config::BasicConfig), detailed properties can be found in it's documentation.
///
/// Although this crate considers the used configuration secure, your project might have different
/// requirements. It is therefore highly recommended however to set up your own configuration to
/// meet your requirements. This can easily be done by choosing any of the configuration structs
/// in the [`config`](::config) module such as [`BasicConfig`](config::BasicConfig), which has a
/// [builder](config::BasicConfigBuilder) available.  
/// Or build your own configuration type with support for converting it into a
/// [`Scheme`](scheme::Scheme) by implementing the [`ToScheme`](scheme::ToScheme) trait.
///
/// A configuration instance is created each time this method is invoked. For generating multiple
/// passphrases it is recommended to build a [`Scheme`](scheme::Scheme) instead as it's much more
/// performant.
///
/// # Entropy
///
/// To figure out what entropy these passphrases have, use:
///
/// ```rust
/// extern crate chbs;
/// use chbs::{config::BasicConfig, prelude::*};
///
/// let entropy = BasicConfig::default().to_scheme().entropy();
/// println!("passphrase() entropy: {:?}", entropy);
/// ```
pub fn passphrase() -> String {
    BasicConfig::default().to_scheme().generate()
}

#[cfg(test)]
mod tests {
    use passphrase;

    /// How many times to iterate for small or infinite tests.
    const ITERS: usize = 32;

    /// Generating a passphrase must produce a string of at least 10 characters.
    #[test]
    fn passphrase_len() {
        assert!(
            passphrase().len() >= 10,
            "passphrase generated by defaults helper is too short"
        );
    }

    /// Repeatedly generating passphrases should produce somewhat unique results.
    #[test]
    fn passphrase_unique() {
        // Generate phrases with helper and dedup
        let mut phrases: Vec<String> = (1..=ITERS).map(|_| passphrase()).collect();
        phrases.dedup();

        // There must be at least 2 unique passphrases
        assert!(phrases.len() > 1);
    }
}
