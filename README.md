[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]
[![Newest release on crates.io][crate-version-badge]][crate-link]
[![Documentation][docs-badge]][docs]
[![Number of downloads on crates.io][crate-download-badge]][crate-link]
[![Project license][crate-license-badge]](LICENSE)

[crate-link]: https://crates.io/crates/chbs
[crate-download-badge]: https://img.shields.io/crates/d/chbs.svg
[crate-version-badge]: https://img.shields.io/crates/v/chbs.svg
[crate-license-badge]: https://img.shields.io/crates/l/chbs.svg
[docs]: https://docs.rs/chbs
[docs-badge]: https://docs.rs/chbs/badge.svg
[gitlab-ci-link]: https://gitlab.com/timvisee/chbs/commits/master
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/chbs/badges/master/pipeline.svg

_Note: this crate is still a work in progress, APIs might change until
stabilisation_

# Rust library: Correct Horse Battery Staple
A secure, easy to use, configurable and extendable passphrase generation library
based on a wordlist, generally known as [diceware].

[![xkcd-img]][xkcd]

The name `chbs` is short for the well known "correct horse battery staple"
password which originates from the [XKCD][xkcd] comic shown above.

* [Features](#features)
* [Requirements](#requirements)
* [Todo](#todo)
* [Concepts](#concepts)
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
* Rust 1.42 or higher (with `std`)

## Todo
The following things need to be looked at before stabilization:

* Use secure strings?
* Additional stylers and configuration options:
  * Add numbers
  * Add special characters
  * Different separators
  * Generated words (based on character sequences)
  * Replace characters with similar looking sequences (`a` to `4`, `o` to `()`)

## Concepts
As the passphrase generation system in this crate is thoroughly abstracted it is
important to understand how the concepts used in this crate work.

Here is what is required for passphrase generation:
- A `Scheme` defines how a passphrase is generated. Passphrases are only
  generated through a scheme.
- A `Scheme` contains components which represents how the passphrase is built up
  and styled. Four kinds of components exist, defining the passphrase generation
  steps. For some kinds one must be defined,
  for other kinds any number is fine:
    1.  `WordSetProvider` (`1` required): provides a list of words to use in
        a passphrase.
    2.  `WordStyler` (`>=0` required): styles passphrase words, for example, to
        capitalize.
    3.  `PhraseBuilder` (`1` required): builds a phrase from a set of passphrase
        words.
    4.  `PhraseStyler` (`>=0` required): styles a whole passphrase.

Things to understand:
- Passphrase generation schemes are commonly created by using a configuration
  structure. Such as structure will provide various configurable fields, and
  builds a corresponding scheme based on it for passphrase generation.

The usual steps for generating a passphrase:
- A configuration structure is built and configured, such as `BasicConfig`.
- The configuration struct creates a corresponding passphrase generation scheme.
- The scheme is used to generate as many passphrases as needed.
- Instead, the `passphrase()` helper method may be used to generate a passphrase
  with zero configuration for ease of use.

See, it isn't too difficult, but allows great extensibility. You probably won't
use most of what this crate provides.  
Take a look at `BasicConfig` to see how to configure your first passphrase
generator.

Additional good-to-know things:
- This crate provides a selection of components for specific tasks, custom
  components may be built.
- This crate provides a `WordList` struct to hold a static wordlist, that may
  use a built-in wordlist or loads a wordlist from a specified file.
- A `WordSampler` may be constructed based on a `WordList` to allow randomized
  word sampling in an uniform manner. Such a sampler is usually what is used as
  word provider in a configuration struct.

## Examples
Here are some basic examples on how to use this crate.

Add `chbs` as dependency in your `Cargo.toml` first:

```toml
[dependencies]
chbs = "0.0.8"
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
let sampler = words.sampler().into_iter();

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
