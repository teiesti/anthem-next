use crate::{
    parsing::PestParser,
    syntax_tree::asp::{BinaryOperator, Constant, Term, UnaryOperator, Variable},
};

mod internal {
    use pest::pratt_parser::PrattParser;

    #[derive(pest_derive::Parser)]
    #[grammar = "parsing/asp/grammar.pest"]
    pub struct Parser;

    lazy_static::lazy_static! {
        pub static ref PRATT_PARSER: PrattParser<Rule> = {
            use pest::pratt_parser::{Assoc::*, Op};
            use Rule::*;

            PrattParser::new()
                .op(Op::infix(interval, Left))
                .op(Op::infix(add, Left) | Op::infix(subtract, Left))
                .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
                .op(Op::prefix(negative))
        };
    }
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

pub struct UnaryOperatorParser;

impl PestParser for UnaryOperatorParser {
    type Node = UnaryOperator;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::unary_operator;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::negative => UnaryOperator::Negative,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct BinaryOperatorParser;

impl PestParser for BinaryOperatorParser {
    type Node = BinaryOperator;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::binary_operator;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::add => BinaryOperator::Add,
            internal::Rule::subtract => BinaryOperator::Subtract,
            internal::Rule::multiply => BinaryOperator::Multiply,
            internal::Rule::divide => BinaryOperator::Divide,
            internal::Rule::modulo => BinaryOperator::Modulo,
            internal::Rule::interval => BinaryOperator::Interval,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct TermParser;

impl PestParser for TermParser {
    type Node = Term;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        internal::PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                internal::Rule::term => TermParser::translate_pair(primary),
                internal::Rule::constant => Term::Constant(ConstantParser::translate_pair(primary)),
                internal::Rule::variable => Term::Variable(VariableParser::translate_pair(primary)),
                _ => Self::report_unexpected_pair(primary),
            })
            .map_prefix(|op, arg| Term::UnaryOperation {
                op: UnaryOperatorParser::translate_pair(op),
                arg: Box::new(arg),
            })
            .map_infix(|lhs, op, rhs| Term::BinaryOperation {
                op: BinaryOperatorParser::translate_pair(op),
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
            .parse(pair.into_inner())
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
