use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub mod asp;
pub mod fol;

pub trait Node: Clone + Debug + Eq + PartialEq + FromStr + Display {}

macro_rules! impl_node {
    ($node:ty, $format:expr, $parser:ty) => {
        impl Node for $node {}

        impl std::fmt::Display for $node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", $format(self))
            }
        }

        impl std::str::FromStr for $node {
            type Err = <$parser as crate::parsing::Parser>::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$parser as crate::parsing::Parser>::parse(s)
            }
        }
    };
}

pub(crate) use impl_node;
