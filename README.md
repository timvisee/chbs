# Correct Horse Battery Staple
A crate for secure passphrase generation based on a wordlist.
This library uses cryptographically secure randomization, and may be used
for generating secret passphrases.

[![xkcd-img]][xkcd]

Notes:
* this crate is still in development, and should thus be used with care
* no waranty is provided for the quality of the passwords generated
  through this library

[xkcd]: https://xkcd.com/936/
[xkcd-img]: https://imgs.xkcd.com/comics/password_strength.png

## Examples
Here are some basic examples on how to use this crate.

Generate a passphrase using the helper function consisting of 5 words
([passphrase.rs](examples/passphrase.rs)):

```rust
extern crate chbs;
use chbs::passphrase;

println!("Passphrase: {:?}", passphrase(5));
```

Use a word sampler to generate an infinite number of random words
((sampler.rs)[examples/sampler.rs]):

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
