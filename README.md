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

TODO before stabilization which will require API changes:
* use secure strings
* allow using custom wordlists
* ability to configure various passphrase generation properties:
  * random word capitalisation
  * add numbers
  * add special characters
  * different separators
  * unique words
* calculate entropy

## Examples
Here are some basic examples on how to use this crate.

Add `chbs` as dependency in your `Cargo.toml` first:

```toml
[dependencies]
chbs = "0.0.1"
```

Generate a passphrase using the helper function consisting of 5 words
([passphrase.rs](examples/passphrase.rs)):  

```rust
extern crate chbs;
use chbs::passphrase;

println!("Passphrase: {:?}", passphrase(5));
```

Run it using `cargo run --example passphrase`.

Use a word sampler to generate an infinite number of random words
([sampler.rs](examples/sampler.rs)):

```rust
extern crate chbs;
use chbs::word_sampler;

let sampler = word_sampler();

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
