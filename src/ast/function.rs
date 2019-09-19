mod aggregate_to_string;
mod cast;
mod count;
mod row_number;

pub use aggregate_to_string::*;
pub use cast::*;
pub use count::*;
pub use row_number::*;

use super::DatabaseValue;
use std::borrow::Cow;

/// A database function definition
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub(crate) typ_: FunctionType<'a>,
    pub(crate) alias: Option<Cow<'a, str>>,
}

/// A database function type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionType<'a> {
    RowNumber(RowNumber<'a>),
    Cast(Cast<'a>),
    Count(Count<'a>),
    AggregateToString(AggregateToString<'a>),
}

impl<'a> Function<'a> {
    /// Give the function an alias in the query.
    pub fn alias<S>(mut self, alias: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.alias = Some(alias.into());
        self
    }
}

macro_rules! function {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind<'a>> for Function<'a> {
                #[inline]
                fn from(f: $kind<'a>) -> Self {
                    Function {
                        typ_: FunctionType::$kind(f),
                        alias: None,
                    }
                }
            }

            impl<'a> From<$kind<'a>> for DatabaseValue<'a> {
                #[inline]
                fn from(f: $kind<'a>) -> Self {
                    Function::from(f).into()
                }
            }
        )*
    );
}

function!(RowNumber, Cast, Count, AggregateToString);
