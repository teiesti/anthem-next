use {
    anyhow::{Context, Result},
    std::{
        fmt::{Debug, Display},
        fs::read_to_string,
        hash::Hash,
        path::Path,
        str::FromStr,
    },
};

pub mod asp;
pub mod fol;

pub trait Node: Clone + Debug + Eq + PartialEq + FromStr + Display + Hash {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        <Self as FromStr>::Err: std::error::Error + Sync + Send + 'static,
    {
        let path = path.as_ref();
        read_to_string(path)
            .with_context(|| format!("could not read file `{}`", path.display()))?
            .parse()
            .with_context(|| format!("could not parse file `{}`", path.display()))
    }
}

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
