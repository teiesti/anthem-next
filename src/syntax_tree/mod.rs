pub mod asp;
pub mod fol;

use {
    pest::{
        error::Error,
        iterators::{Pair, Pairs},
        Parser, RuleType,
    },
    std::{
        any::type_name,
        fmt::{Debug, Display},
        str::FromStr,
    },
};

pub trait Node:
    Clone
    + Display
    + Debug
    + Eq
    + PartialEq
    + for<'a> From<Pair<'a, Self::Rule>>
    + for<'a> From<Pairs<'a, Self::Rule>>
    + FromStr<Err = Error<Self::Rule>>
{
    type Parser: Parser<Self::Rule>;
    type Rule: RuleType;
    const RULE: Self::Rule;
}

macro_rules! impl_from_pairs {
    ($node: path) => {
        impl
            std::convert::From<pest::iterators::Pairs<'_, <Self as crate::syntax_tree::Node>::Rule>>
            for $node
        {
            fn from(
                mut pairs: pest::iterators::Pairs<'_, <Self as crate::syntax_tree::Node>::Rule>,
            ) -> Self {
                let pair = pairs
                    .next()
                    .unwrap_or_else(|| crate::syntax_tree::report_missing_pair::<Self>());
                if let Some(pair) = pairs.next() {
                    crate::syntax_tree::report_unexpected_pair::<Self, _>(pair)
                };
                Self::from(pair)
            }
        }
    };
}

pub(crate) use impl_from_pairs;

macro_rules! impl_from_str {
    ($node: path) => {
        impl std::str::FromStr for $node {
            type Err = pest::error::Error<<Self as crate::syntax_tree::Node>::Rule>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use crate::parsing::CompleteParser as _;
                let pairs = Parser::parse_complete(Self::RULE, s)?;
                Ok(Self::from(pairs))
            }
        }
    };
}

pub(crate) use impl_from_str;

pub fn report_missing_pair<T>() -> ! {
    panic!("in {}: no pair found", type_name::<T>())
}

pub fn report_unexpected_pair<T, Rule: RuleType>(pair: Pair<'_, Rule>) -> ! {
    panic!("in {}: unexpected pair found: {pair}", type_name::<T>())
}
