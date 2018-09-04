//! Various [`Scheme`](::scheme::Scheme) components to define passphrase generation
//!
//! These components are used in a [`Scheme`](::scheme::Scheme) to define how passphrases are
//! generated. Components are used for providing a set of words, for combining words into a
//! passphrase and for styling words and passphrases. These types are defined as trait, to allow
//! implementing custom components in your own crate to extend passphrase generation
//! functionallity.
//!
//! The available component kind traits are defined in the [`traits`](self::traits) module and
//! are listed below:
//!
//! - [`WordSetProvider`](self::traits::WordSetProvider)
//! - [`WordStyler`](self::traits::WordStyler)
//! - [`PhraseBuilder`](self::traits::PhraseBuilder)
//! - [`PhraseStyler`](self::traits::PhraseStyler)
//!
//! The modules [`word`](self::word) and [`phrase`](self::phrase) contains various included
//! components to use. For example, the [`WordCapitalizer`](self::word::WordCapitalizer) component
//! may be used to capitalize passphrase words as configured.

// Re-export the modules
pub mod phrase;
pub mod traits;
pub mod word;
