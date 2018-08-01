# Rust library: Correct Horse Battery Staple
A crate providing secure passphrase generation based on a wordlist also known as
[diceware].

[![xkcd-img]][xkcd]

The name `chbs` is short for the well known "Correct Horse Battery Staple"
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
([passphrase.rs](examples/passphrase.rs), `cargo run --example passphrase`):

```rust
extern crate chbs;
use chbs::passphrase;

println!("Passphrase: {:?}", passphrase(5));
```

Use a word sampler to generate an infinite number of random words
([sampler.rs](examples/sampler.rs), `cargo run --example sampler`):

```rust
extern crate chbs;
use chbs::word_sampler;

let mut sampler = word_sampler();

for i in 0..8 {
    println!(
        "Sampled word #{}: {:?}",
        i,
        sampler.next().unwrap(),
    );
}
```

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.

[diceware]: https://en.wikipedia.org/wiki/Diceware
[xkcd]: https://xkcd.com/936/
[xkcd-img]: https://imgs.xkcd.com/comics/password_strength.png
