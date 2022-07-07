//! _Note: this crate is still a work in progress, APIs might change until
//! stabilisation_
//!
//! A secure, easy to use, configurable and extendable passphrase generation library
//! based on a wordlist, generally known as [diceware].
//!
//! The crate name `chbs` is short for the well known "correct horse battery staple" password
//! which originates from an [XKCD][xkcd] comic shown in the README [here][xkcd_comic].
//!
//! This library uses cryptographically secure randomization, and may be used
//! for generating secret passphrases.
//! Please refer to the [README][readme] for more information on security.
//!
//! ## Concepts
//! As the passphrase generation system in this crate is thoroughly abstracted it is
//! important to understand how the concepts used in this crate work.
//!
//! Here is what is required for passphrase generation:
//! - A [`Scheme`](scheme::Scheme) defines how a passphrase is generated. Passphrases are only
//!   generated through a scheme.
//! - A [`Scheme`](scheme::Scheme) contains components which represents how the passphrase is built
//!   up and styled. Four kinds of components exist, defining the passphrase generation
//!   steps. For some kinds one must be defined,
//!   for other kinds any number is fine:
//!     1.  [`WordSetProvider`](component::traits::WordSetProvider) (`1` required):
//!         provides a list of words to use in a passphrase.
//!     2.  [`WordStyler`](component::traits::WordStyler) (`>=0` required):
//!         styles passphrase words, for example, to capitalize.
//!     3.  [`PhraseBuilder`](component::traits::PhraseBuilder) (`1` required):
//!         builds a phrase from a set of passphrase words.
//!     4.  [`PhraseStyler`](component::traits::PhraseBuilder) (`>=0` required):
//!         styles a whole passphrase.
//!
//! Things to understand:
//! - Passphrase generation schemes are commonly created by using a configuration
//!   structure. Such as structure will provide various configurable fields, and
//!   builds a corresponding scheme based on it for passphrase generation.
//!
//! The usual steps for generating a passphrase:
//! - A configuration structure is built and configured,
//!   such as [`BasicConfig`](config::BasicConfig).
//! - The configuration struct creates a corresponding passphrase generation scheme.
//! - The scheme is used to generate as many passphrases as needed.
//! - Instead, the [`passphrase()`](passphrase) helper method may be used to generate a passphrase
//!   with zero configuration for ease of use.
//!
//! See, it isn't too difficult, but allows great extensibility. You probably won't
//! use most of what this crate provides.  
//! Take a look at [`BasicConfig`](config::BasicConfig) to see how to configure your first
//! passphrase generator.
//!
//! Additional good-to-know things:
//! - This crate provides a selection of components for specific tasks, custom
//!   components may be built.
//! - This crate provides a [`WordList`](word::WordList) struct to hold a static wordlist,
//!   that may use a built-in wordlist or loads a wordlist from a specified file.
//! - A [`WordSampler`](word::WordSampler) may be [constructed](word::WordList::sampler) based
//!   on a [`WordList`](word::WordList) to allow randomized word sampling in an uniform manner.
//!   Such a sampler is usually what is used as word provider in a configuration struct.
//!
//! ## Examples
//! Here are two very basic examples.
//! First to generate a passphrase with zero configuration using a helper function applying
//! library defaults ([src][example_passphrase]):
//!
//! ```rust
//! use chbs::passphrase;
//!
//! println!("Passphrase: {:?}", passphrase());
//! ```
//!
//! Generating a passphrase with configuration is recommended, here is a basic
//! example ([src][example_passphrase_config]):
//!
//! ```rust
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
//! More examples are available in the documentation throughout the crate,
//! and in the [`./examples`][examples] directory.
//!
//! ## More information
//! Please reference to the [README][readme] in the [code repository][repo] for more information.
//!
//! [diceware]: https://en.wikipedia.org/wiki/Diceware
//! [examples]: https://gitlab.com/timvisee/chbs/tree/master/examples
//! [example_passphrase]: https://gitlab.com/timvisee/chbs/blob/master/examples/passphrase.rs
//! [example_passphrase_config]: https://gitlab.com/timvisee/chbs/blob/master/examples/passphrase_config.rs
//! [readme]: https://gitlab.com/timvisee/chbs/blob/master/README.md
//! [repo]: https://gitlab.com/timvisee/chbs
//! [xkcd]: https://xkcd.com/936/
//! [xkcd_comic]: https://gitlab.com/timvisee/chbs#rust-library-correct-horse-battery-staple

#[macro_use]
extern crate derive_builder;
extern crate rand;

use crate::config::BasicConfig;
use crate::prelude::*;

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

/// Zero-configuration passphrase generation helper
///
/// A quick way to generate a passphrase with no configuration.  
/// Passphrases are based on the `default()` of a [`BasicConfig`](config::BasicConfig),
/// detailed properties can be found in it's documentation.
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
    use std::sync::mpsc::RecvError;
    use std::sync::{mpsc::channel, mpsc::Sender, Arc};
    use std::thread;

    use super::config::BasicConfig;
    use super::passphrase;
    use super::scheme::{Scheme, ToScheme};
    use super::word::WordList;

    /// How many times to iterate for small or infinite tests.
    const ITERS: usize = 32;

    /// Generating a passphrase must produce a string of at least 10 characters.
    #[test]
    fn passphrase_len() {
        assert!(
            passphrase().len() >= 10,
            "passphrase generated by defaults helper is too short",
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

    #[test]
    fn sampler_into_iterator() {
        let words = WordList::default();
        let iterator = words.sampler().into_iter();
        let result: Vec<String> = iterator.take(8).collect();
        assert_eq!(8, result.len());
    }

    #[test]
    fn threading() -> Result<(), RecvError> {
        let scheme = Arc::new(BasicConfig::default().to_scheme());
        let (tx, rx) = channel::<String>();

        let handle1 = spawn_thread(scheme.clone(), tx.clone());
        let handle2 = spawn_thread(scheme.clone(), tx.clone());

        handle1.join().unwrap();
        handle2.join().unwrap();
        std::mem::drop(tx);

        rx.recv()?;
        rx.recv()?;
        Ok(())
    }

    fn spawn_thread(scheme: Arc<Scheme>, tx: Sender<String>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            tx.send(scheme.generate()).unwrap();
        })
    }
}
