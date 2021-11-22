//! Passphrase generation with basic configuration
//!
//! This example shows how to use the [`BasicConfig`](config::BasicConfig) structure for
//! configuring how passphrases should be generated. See the [`BasicConfig`](config::BasicConfig)
//! documentation for all available options.
//!
//! Other configuration structures are available in the [`config`](config) module.
//! You may build your own configuration structure with support for converting it into a
//! [`Scheme`](scheme::Scheme) by implementing the [`ToScheme`](scheme::ToScheme) trait.
//!
//! Note that this example prints the generated passphrase in the console,
//! which might not be desired in most situations.

use chbs::{config::BasicConfig, prelude::*, probability::Probability};

fn main() {
    // Build a custom configuration to:
    // - use hyphens as separator
    // - sometimes capitalizes whole passphrase words
    // - sometimes capitalizes the first characters of passphrase words
    // TODO: use the builder instead
    // let config = BasicConfigBuilder::default()
    //     .separator("-")
    //     .capitalize_first(Probability::from(0.33))
    //     .capitalize_words(Probability::half())
    //     .build()
    //     .unwrap();
    let mut config = BasicConfig::default();
    config.words = 8;
    config.separator = "-".into();
    config.capitalize_first = Probability::from(0.33);
    config.capitalize_words = Probability::half();

    let scheme = config.to_scheme();

    println!("Passphrase: {:?}", scheme.generate());
    println!("Entropy: {:?}", scheme.entropy().bits());
}
