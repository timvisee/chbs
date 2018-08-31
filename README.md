[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]
[![Newest release on crates.io][crate-version-badge]][crate-link]
[![Number of downloads on crates.io][crate-download-badge]][crate-link]
[![Project license][crate-license-badge]](LICENSE)

[crate-link]: https://crates.io/crates/chbs
[crate-download-badge]: https://img.shields.io/crates/d/chbs.svg
[crate-version-badge]: https://img.shields.io/crates/v/chbs.svg
[crate-license-badge]: https://img.shields.io/crates/l/chbs.svg
[gitlab-ci-link]: https://gitlab.com/timvisee/chbs/commits/master
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/chbs/badges/master/pipeline.svg

# Rust library: Correct Horse Battery Staple
A crate providing secure passphrase generation based on a wordlist also known as
[diceware].

[![xkcd-img]][xkcd]

The name `chbs` is short for the well known "correct horse battery staple"
password which originates from the [XKCD][xkcd] comic shown above.

This library uses cryptographically secure randomization, and may be used
for generating secret passphrases.

Notes:
* this crate is still in development, and should thus be used with care
* no warranty is provided for the quality of the passwords generated
  through this library

Features:
* simple and secure passphrase generation
* configurable passphrase generation schemes to meet your requirements
* use builtin or custom wordlists
* calculate passphrase entropy
* easy to use abstracted generation API
* very extendable, to set it up it any way you like

TODO before stabilization which will require API changes:
* use secure strings
* ability to configure various passphrase generation properties:
  * add numbers
  * add special characters
  * different separators
  * generated words (based on character sequences)

## Examples
Here are some basic examples on how to use this crate.

Add `chbs` as dependency in your `Cargo.toml` first:

```toml
[dependencies]
chbs = "0.0.1"
```

Generate a passphrase with zero configuration using a helper function applying
library defaults ([passphrase.rs](examples/passphrase.rs)):

```rust
extern crate chbs;
use chbs::passphrase;

println!("Passphrase: {:?}", passphrase());
```

Run it using `cargo run --example passphrase`.

Generating a passphrase with configuration is recommended, here is a basic
example ([`passphrase_config.rs`](examples/passphrase_config.rs)):

```rust
extern crate chbs;
use chbs::{config::BasicConfig, prelude::*, probability::Probability};

// Build a custom configuration to:
let mut config = BasicConfig::default();
config.words = 8;
config.separator = "-".into();
config.capitalize_first = Probability::from(0.33);
config.capitalize_words = Probability::half();
let mut scheme = config.to_scheme();

println!("Passphrase: {:?}", scheme.generate());
println!("Entropy: {:?}", scheme.entropy().bits());
```

Run it using `cargo run --example passphrase_config`.

Use a word sampler to generate an infinite number of random words based on
a wordlist ([sampler.rs](examples/sampler.rs)):

```rust
extern crate chbs;
use chbs::word::WordList;

let words = WordList::default();
let sampler = words.sampler();

for word in sampler.take(8) {
    println!("Sampled word: {:?}", word);
}
```

Run it using `cargo run --example sampler`.

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.

[diceware]: https://en.wikipedia.org/wiki/Diceware
[xkcd]: https://xkcd.com/936/
[xkcd-img]: https://imgs.xkcd.com/comics/password_strength.png
