//! The conventional commit type and its simple, and typed implementations.

pub mod simple;
pub mod typed;

use crate::component::{Body, Description, Scope, Trailer, Type};
use crate::error::Error;
use crate::parser::parse;
use nom::error::VerboseError;
use std::fmt;

/// A conventional commit.
#[derive(Clone, Debug)]
pub struct Commit<'a> {
    ty: Type<'a>,
    scope: Option<Scope<'a>>,
    description: Description<'a>,
    body: Option<Body<'a>>,
    breaking: bool,
    trailers: Vec<Trailer<'a>>,
}

impl<'a> Commit<'a> {
    /// Create a new Conventional Commit based on the provided commit message
    /// string.
    ///
    /// # Errors
    ///
    /// This function returns an error if the commit does not conform to the
    /// Conventional Commit specification.
    pub fn new(string: &'a str) -> Result<Self, Error> {
        let (ty, scope, breaking, description, body, trailers) =
            parse::<VerboseError<&'a str>>(string).map_err(|err| (string, err))?;

        Ok(Self {
            ty: ty.into(),
            scope: scope.map(Into::into),
            description: description.into(),
            body: body.map(Into::into),
            breaking: breaking.is_some()
                || trailers.iter().any(|(k, _, _)| k == &"BREAKING CHANGE"),
            trailers: trailers.into_iter().map(Into::into).collect(),
        })
    }
}

impl fmt::Display for Commit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::Simple;

        f.write_str(self.type_())?;

        if let Some(scope) = &self.scope() {
            f.write_fmt(format_args!("({})", scope))?;
        }

        f.write_fmt(format_args!(": {}", &self.description()))?;

        if let Some(body) = &self.body() {
            f.write_fmt(format_args!("\n\n{}", body))?;
        }

        for t in self.trailers() {
            write!(f, "\n\n{}{}{}", t.key(), t.separator(), t.value())?;
        }

        Ok(())
    }
}
