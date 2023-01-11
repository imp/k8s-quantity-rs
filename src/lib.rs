#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::flat_map_option)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use std::error;
use std::fmt;
use std::num;
use std::str;

pub use amount::Int64Amount;
pub use format::Format;
pub use scale::InvalidScale;
pub use scale::Scale;

mod amount;
mod format;
mod impls;
mod scale;

#[derive(Clone, Debug)]
pub struct Quantity {
    int: Int64Amount,
    format: Format,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidQuantity(num::ParseIntError);

impl From<num::ParseIntError> for InvalidQuantity {
    fn from(value: num::ParseIntError) -> Self {
        Self(value)
    }
}

impl fmt::Display for InvalidQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for InvalidQuantity {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests;
