//! Passphrase entropy related structures
//!
//! This module provides the [`Entropy`](Entropy) type that is used to define passphrase generation
//! entropy. [Components](::component) use this type to accumulate their entropy for a final total.
//!
//! The [`HasEntropy`](HasEntropy) trait may be implemented on types that provide some sort of
//! entropy. Implementing this is required on [components](::component) to allow entropy
//! calculation on a configured [`Scheme`](::scheme::Scheme).

use std::{
    fmt::{self, Display, Formatter},
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

/// Password entropy.
///
/// The entropy number used internally represents the number of base 2 entropy bits,
/// and is calculated using `log2(choices)`.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Entropy(f64);

impl Entropy {
    /// Construct entropy for zero bits, representing no entropy.
    pub fn zero() -> Self {
        Entropy(0.0)
    }

    /// Construct entropy for one bit, representing 50/50 chance.
    pub fn one() -> Self {
        Entropy(1.0)
    }

    /// Construct entropy from a number of entropy bits.
    pub fn from_bits<F: Into<f64>>(bits: F) -> Self {
        Entropy(bits.into())
    }

    /// Construct entropy from a real number.
    ///
    /// For a wordlist of 7776 words where choices are uniform, the real number `7776` may be given
    /// to construct the proper entropy value. This would produce an entropy instance with about
    /// `12.9` bits.
    ///
    /// If `Entropy` should be constructed from a number of bits, use
    /// [`from_bits`](Entropy::from_bits) instead.
    pub fn from_real<F: Into<f64>>(real: F) -> Self {
        Entropy(real.into().log2())
    }

    /// Get the number of entropy bits.
    pub fn bits(self) -> f64 {
        self.0
    }
}

impl Display for Entropy {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} bits", self.bits())
    }
}

impl Sum for Entropy {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator,
        I::Item: Into<Entropy>,
    {
        iter.fold(Entropy::zero(), |total, e| total + e.into())
    }
}

/// A macro to derive common operator traits for a struct.
macro_rules! derive_ops {
    (impl $trait_: ident for $type_: ident { fn $method: ident }) => {
        // Operation with another Entropy object
        impl $trait_<$type_> for $type_ {
            type Output = $type_;

            #[inline]
            fn $method(self, $type_(b): $type_) -> $type_ {
                let $type_(a) = self;
                $type_(a.$method(&b))
            }
        }

        // Operation with another integer or float value
        impl<B> $trait_<B> for $type_
        where
            B: Into<f64>,
        {
            type Output = $type_;

            #[inline]
            fn $method(self, b: B) -> $type_ {
                let $type_(a) = self;
                $type_(a.$method(&b.into()))
            }
        }
    };
}

derive_ops! { impl Add for Entropy { fn add } }
derive_ops! { impl Sub for Entropy { fn sub } }
derive_ops! { impl Mul for Entropy { fn mul } }
derive_ops! { impl Div for Entropy { fn div } }

/// An entropy source.
///
/// Get the entropy value for the current component, whether that is a word styler, a phrase
/// builder or something else.
///
/// TODO: properly describe what entropy is here.
pub trait HasEntropy {
    /// Get the entropy value for this whole component.
    /// The returned entropy value may be accumulated from various internal entropy sources.
    ///
    /// See the documentation on [Entropy](Entropy) for details on what entropy is and how it
    /// should be calculated.
    /// If this component does not have any effect on passphrase entropy `1` should be returned.
    fn entropy(&self) -> Entropy;
}
