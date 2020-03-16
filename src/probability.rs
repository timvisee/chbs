//! Probability related strucutres
//!
//! This module provides the [`Probability`](Probability) type that is used to define the
//! probability of something being true. This is commonly used in passphrase generation
//! [components](::component) for randomisation of passphrase styling features.
//! For example, the [`WordCapitalizer`](::component::word::WordCapitalizer) uses this type in it's
//! fields.
//!
//! See [`Probability`](Probability) for more details.

use rand::{prelude::*, thread_rng};

use crate::entropy::Entropy;
use crate::prelude::*;

/// A probability definition.
///
/// This defines what the probability is of something being true.
///
/// The function [`gen_bool`](Probability::gen_bool) can be used to generate a boolean based on
/// this probability. Depending on what randomness source is given, it may be cryptographically
/// secure.
#[derive(Copy, Clone, Debug)]
pub enum Probability {
    /// This is always true.
    Always,

    /// This is sometimes true.
    ///
    /// If `1.0` it's always true, if `0.0` it is never true, the value may be anywhere in between.
    ///
    /// If the value is exactly `0.0` or `1.0` the variants [`Always`](Probability::Always) and
    /// [`Never`](Probability::Never) should be used instead.
    /// It is therefore recommended to construct this type using the
    /// [`from`](Probability::from) method as this automatically chooses the correct variant.
    ///
    /// This value may never be `p < 0` or `p > 1`, as it will cause panics.
    Sometimes(f64),

    /// This is never true, and is always false.
    Never,
}

impl Probability {
    /// Construct a probability from the given probability value.
    ///
    /// If `1.0` it's always true, if `0.0` it is never true.
    /// Values outside this range will be wrapped to their corresponding edge.
    pub fn from(probability: f64) -> Self {
        match probability {
            x if x >= 1.0 => Probability::Always,
            x if x <= 0.0 => Probability::Never,
            x => Probability::Sometimes(x),
        }
    }

    /// Construct a probability from the given percentage.
    ///
    /// If `100.0` it's always true, if `0.0` it is never true.
    /// Values outside this range will be wrapped to their corresponding edge.
    pub fn from_percentage(percentage: f64) -> Self {
        match percentage {
            x if x >= 100.0 => Probability::Always,
            x if x <= 0.0 => Probability::Never,
            x => Probability::Sometimes(x / 100.0),
        }
    }

    /// Construct a probability that is true half of the times (50/50, 50%).
    pub fn half() -> Self {
        Self::from(0.5)
    }

    /// Get the probability value.
    ///
    /// To get the percentage, use [`percentage`](Probability::percentage).
    pub fn value(&self) -> f64 {
        match self {
            Probability::Always => 1.0,
            Probability::Sometimes(p) => *p,
            Probability::Never => 0.0,
        }
    }

    /// Get the probability percentage.
    ///
    /// To get the probability value, use [`value`](Probability::value).
    pub fn percentage(&self) -> f64 {
        self.value() * 100.0
    }

    /// Generate a boolean for this probability.
    ///
    /// If the given randomness source to `rng` is cryptographically secure,
    /// the generated boolean can be considered cryptographically secure as well.
    pub fn gen_bool<R: Rng>(self, rng: &mut R) -> bool {
        match self {
            Probability::Always => true,
            Probability::Never => false,
            Probability::Sometimes(p) => rng.gen_bool(p),
        }
    }

    /// Generate a cryptographically secure boolean for this probability.
    ///
    /// This method obtains a cryptographically secure randomness source through `thread_rng`
    /// provided by the `rand` crate and generates a boolean through
    /// [`gen_bool`](Probability::gen_bool).
    pub fn gen_bool_secure(self) -> bool {
        match self {
            Probability::Always => true,
            Probability::Never => false,
            Probability::Sometimes(_) => self.gen_bool(&mut thread_rng()),
        }
    }
}

impl HasEntropy for Probability {
    fn entropy(&self) -> Entropy {
        match self {
            // TODO: properly calculate entropy here
            Probability::Sometimes(_p) => Entropy::one(),
            _ => Entropy::zero(),
        }
    }
}

/// Allow easy `Probability` selection of `Always` and `Never` from a boolean.
impl From<bool> for Probability {
    fn from(b: bool) -> Probability {
        if b {
            Probability::Always
        } else {
            Probability::Never
        }
    }
}
