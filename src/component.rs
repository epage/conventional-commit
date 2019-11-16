//! Conventional Commit components.

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Error, ErrorKind};

/// A single footer.
///
/// A footer is similar to a Git trailer, with the exception of not requiring
/// whitespace before newlines.
///
/// See: <https://git-scm.com/docs/git-interpret-trailers>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Footer<'a> {
    token: FooterToken<'a>,
    sep: FooterSeparator,
    value: FooterValue<'a>,
}

impl<'a> Footer<'a> {
    /// Piece together a footer.
    pub const fn new(token: FooterToken<'a>, sep: FooterSeparator, value: FooterValue<'a>) -> Self {
        Self { token, sep, value }
    }

    /// The token of the footer.
    pub const fn token(&self) -> FooterToken<'a> {
        self.token
    }

    /// The separator between the footer token and its value.
    pub const fn separator(&self) -> FooterSeparator {
        self.sep
    }

    /// The value of the footer.
    pub const fn value(&self) -> FooterValue<'a> {
        self.value
    }
}

/// The "simple footer" variant, for convenient access to the string slice
/// values of its components.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct SimpleFooter<'a> {
    pub(crate) footer: &'a Footer<'a>,
}

impl<'a> SimpleFooter<'a> {
    /// The token of the footer.
    pub fn token(&self) -> &str {
        &*self.footer.token
    }

    /// The separator between the footer token and its value.
    pub fn separator(&self) -> &str {
        &*self.footer.sep
    }

    /// The value of the footer.
    pub fn value(&self) -> &str {
        &*self.footer.value
    }
}

/// The type of separator between the footer token and value.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[cfg_attr(feature = "serde", serde(into = "&'static str"))]
pub enum FooterSeparator {
    /// ": "
    ColonSpace,

    /// " #"
    SpacePound,

    #[doc(hidden)]
    __NonExhaustive,
}

impl FooterSeparator {
    /// The string representation of the footer.
    pub fn as_str(&self) -> &'static str {
        match self {
            FooterSeparator::ColonSpace => ": ",
            FooterSeparator::SpacePound => " #",
            FooterSeparator::__NonExhaustive => unreachable!(),
        }
    }
}

impl AsRef<str> for FooterSeparator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for FooterSeparator {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Into<&'static str> for FooterSeparator {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl fmt::Display for FooterSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl FromStr for FooterSeparator {
    type Err = Error;

    fn from_str(sep: &str) -> Result<Self, Self::Err> {
        match sep {
            ": " => Ok(FooterSeparator::ColonSpace),
            " #" => Ok(FooterSeparator::SpacePound),
            _ => Err(Error::new(ErrorKind::InvalidFormat)),
        }
    }
}

impl<'s> TryFrom<&'s str> for FooterSeparator {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

macro_rules! components {
($($ty:ident),+) => (
    $(
        /// A component of the conventional commit.
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $ty<'a> {
            #[cfg_attr(feature = "serde", serde(borrow))]
            value: &'a str
        }

        impl<'a> $ty<'a> {
            /// Create a $ty
            pub fn new(value: &'a str) -> Self {
                Self { value }
            }
        }

        impl Deref for $ty<'_> {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl fmt::Display for $ty<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.value.fmt(f)
            }
        }

        impl<'a> From<&'a str> for $ty<'a> {
            fn from(value: &'a str) -> Self {
                Self { value }
            }
        }
    )+
)
}

macro_rules! unicase_components {
    ($($ty:ident),+) => (
        $(
            /// A component of the conventional commit.
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
            pub struct $ty<'a> {
                value: unicase::UniCase<&'a str>
            }

            impl<'a> $ty<'a> {
                /// Create a $ty
                pub fn new(value: &'a str) -> Self {
                    Self { value: unicase::UniCase::new(value) }
                }
            }

            impl Deref for $ty<'_> {
                type Target = str;

                fn deref(&self) -> &Self::Target {
                    &self.value.into_inner()
                }
            }

            impl fmt::Display for $ty<'_> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.value.fmt(f)
                }
            }

            impl<'a> From<&'a str> for $ty<'a> {
                fn from(value: &'a str) -> Self {
                    Self { value: unicase::UniCase::new(value) }
                }
            }
        )+
    )
}

components![Description, Body, FooterValue];

unicase_components![Type, Scope, FooterToken];
