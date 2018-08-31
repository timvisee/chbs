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
A secure, easy to use, configurable and extendable passphrase generation library
based on a wordlist, generally known as [diceware].

[![xkcd-img]][xkcd]

The name `chbs` is short for the well known "correct horse battery staple"
password which originates from the [XKCD][xkcd] comic shown above.

* [Features](#features)
* [Requirements](#requirements)
* [Todo](#todo)
* [Examples](#examples)
* [Additional notes](#additional-notes)
* [License](#license)

This library uses cryptographically secure randomization, and may be used
for generating secret passphrases[*](#additional-notes).

## Features
* Simple and secure passphrase generation
* Configurable generation schemes to meet your requirements
* Use built-in or custom wordlists
* Calculate passphrase entropy
* Easy to use abstracted API
* Very extendable, to set it up it any way you like

## Requirements
* Rust 1.26 or higher (with `std`)

## Todo
The following things need to be looked at before stabilization:

* Use secure strings?
* Additional stylers and configuration options:
  * Add numbers
  * Add special characters
  * Different separators
  * Generated words (based on character sequences)
  * Replace characters with similar looking sequences (`a` to `4`, `o` to `()`)

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

See all examples in the [`./examples`](./examples) directory.

## Additional notes
* This crate is still in development, and should thus be used with care
* No warranty is provided for the quality of the passwords or passphrases
  generated through this library
* Entropy calculations may be faulty at this moment

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.

[diceware]: https://en.wikipedia.org/wiki/Diceware
[xkcd]: https://xkcd.com/936/
[xkcd-img]: https://imgs.xkcd.com/comics/password_strength.png
