use {
    super::{impl_from_pairs, impl_from_str, report_unexpected_pair, Node},
    crate::parsing::asp::{Parser, Rule},
    pest::iterators::Pair,
    std::fmt::{self, Display},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Constant {
    Infimum,
    Integer(isize),
    Symbol(String),
    Supremum,
}

impl Node for Constant {
    type Parser = Parser;
    type Rule = Rule;
    const RULE: Self::Rule = Rule::constant;
}

impl Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infimum => write!(f, "#inf"),
            Self::Integer(n) => write!(f, "{n}"),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::Supremum => write!(f, "#sup"),
        }
    }
}

impl From<Pair<'_, Rule>> for Constant {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::infimum => Constant::Infimum,
            Rule::integer => Constant::Integer(pair.as_str().parse().unwrap()),
            Rule::symbol => Constant::Symbol(pair.as_str().into()),
            Rule::supremum => Constant::Supremum,
            _ => report_unexpected_pair::<Self, _>(pair),
        }
    }
}

impl_from_pairs!(Constant);

impl_from_str!(Constant);

// TODO Tobias: Implement the remaining abstract syntax tree for ASP

#[cfg(test)]
mod tests {
    use super::Constant;

    #[test]
    fn parse_constant_infimum() {
        assert_eq!("#inf".parse::<Constant>(), Ok(Constant::Infimum));
        assert_eq!("#infimum".parse::<Constant>(), Ok(Constant::Infimum));
    }

    #[test]
    fn parse_constant_integer() {
        assert_eq!("-1".parse::<Constant>(), Ok(Constant::Integer(-1)));
        assert_eq!("0".parse::<Constant>(), Ok(Constant::Integer(0)));
        assert_eq!("1".parse::<Constant>(), Ok(Constant::Integer(1)));
    }

    #[test]
    fn parse_constant_symbol() {
        assert_eq!("a".parse::<Constant>(), Ok(Constant::Symbol("a".into())));
    }

    #[test]
    fn parse_constant_supremum() {
        assert_eq!("#sup".parse::<Constant>(), Ok(Constant::Supremum));
        assert_eq!("#supremum".parse::<Constant>(), Ok(Constant::Supremum));
    }

    #[test]
    fn fmt_constant_infimum() {
        assert_eq!(Constant::Infimum.to_string(), "#inf");
    }

    #[test]
    fn fmt_constant_integer() {
        assert_eq!(Constant::Integer(-1).to_string(), "-1");
        assert_eq!(Constant::Integer(0).to_string(), "0");
        assert_eq!(Constant::Integer(1).to_string(), "1");
    }

    #[test]
    fn fmt_constant_symbol() {
        assert_eq!(Constant::Symbol("a".into()).to_string(), "a");
    }

    #[test]
    fn fmt_constant_supremum() {
        assert_eq!(Constant::Supremum.to_string(), "#sup");
    }

    // TODO Tobias: Add test for the remaining abstract syntax tree for ASP
}
