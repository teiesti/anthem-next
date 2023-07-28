use crate::{
    parsing::PestParser,
    syntax_tree::asp::{Constant, Variable},
};

mod internal {
    #[derive(pest_derive::Parser)]
    #[grammar = "parsing/asp/grammar.pest"]
    pub struct Parser;
}

pub struct ConstantParser;

impl PestParser for ConstantParser {
    type Node = Constant;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::constant;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::constant => Self::translate_pairs(pair.into_inner()),
            internal::Rule::infimum => Constant::Infimum,
            internal::Rule::integer => Constant::Integer(pair.as_str().parse().unwrap()),
            internal::Rule::symbol => Constant::Symbol(pair.as_str().into()),
            internal::Rule::supremum => Constant::Supremum,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct VariableParser;

impl PestParser for VariableParser {
    type Node = Variable;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::variable;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::variable => Self::translate_pairs(pair.into_inner()),
            internal::Rule::anonymous_variable => Variable::Anonymous,
            internal::Rule::named_variable => Variable::Named(pair.as_str().into()),
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

// TODO Tobias: Continue implementing pest parsing for ASP here

#[cfg(test)]
mod tests {
    use {
        super::{ConstantParser, VariableParser},
        crate::{
            parsing::TestedParser,
            syntax_tree::asp::{Constant, Variable},
        },
    };

    #[test]
    fn parse_constant() {
        ConstantParser
            .should_parse_into([
                ("#inf", Constant::Infimum),
                ("#infimum", Constant::Infimum),
                ("0", Constant::Integer(0)),
                ("1", Constant::Integer(1)),
                ("42", Constant::Integer(42)),
                ("4711", Constant::Integer(4711)),
                ("-1", Constant::Integer(-1)),
                ("a", Constant::Symbol("a".into())),
                ("aa", Constant::Symbol("aa".into())),
                ("aA", Constant::Symbol("aA".into())),
                ("_a", Constant::Symbol("_a".into())),
                ("'a", Constant::Symbol("'a".into())),
                ("_'x'_'x'_", Constant::Symbol("_'x'_'x'_".into())),
                ("noto", Constant::Symbol("noto".into())),
                ("#sup", Constant::Supremum),
                ("#supremum", Constant::Supremum),
            ])
            .should_reject([
                "A", "4 2", "00", "-0", "--1", "a a", "a-a", "'", "not", "#", "#infi", "#supi", "_",
            ]);
    }

    #[test]
    fn parse_variable() {
        VariableParser
            .should_parse_into([
                ("_", Variable::Anonymous),
                ("A", Variable::Named("A".into())),
                ("AA", Variable::Named("AA".into())),
                ("Aa", Variable::Named("Aa".into())),
                ("_A", Variable::Named("_A".into())),
                ("'A", Variable::Named("'A".into())),
                ("_'X'_'X'_", Variable::Named("_'X'_'X'_".into())),
            ])
            .should_reject(["a", "1", "A A", "A-A", "'", "-A"]);
    }

    // TODO Tobias: Add tests for the remaining parsers
}
