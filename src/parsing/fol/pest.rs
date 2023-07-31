use crate::{
    parsing::PestParser,
    syntax_tree::fol::{BasicIntegerTerm, BinaryOperator, GeneralTerm, IntegerTerm, UnaryOperator},
};

mod internal {
    use pest::pratt_parser::PrattParser;

    #[derive(pest_derive::Parser)]
    #[grammar = "parsing/fol/grammar.pest"]
    pub struct Parser;

    lazy_static::lazy_static! {
        pub static ref PRATT_PARSER: PrattParser<Rule> = {
            use pest::pratt_parser::{Assoc::*, Op};
            use Rule::*;

            PrattParser::new()
                .op(Op::infix(add, Left) | Op::infix(subtract, Left))
                .op(Op::infix(multiply, Left))
                .op(Op::prefix(negative))
        };
    }
}

pub struct BasicIntegerTermParser;

impl PestParser for BasicIntegerTermParser {
    type Node = BasicIntegerTerm;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::n_basic_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::n_basic_term => Self::translate_pairs(pair.into_inner()),
            internal::Rule::infimum => BasicIntegerTerm::Infimum,
            internal::Rule::supremum => BasicIntegerTerm::Supremum,
            internal::Rule::numeral => BasicIntegerTerm::Numeral(pair.as_str().parse().unwrap()),
            internal::Rule::n_variable => match pair.into_inner().next() {
                Some(pair) if pair.as_rule() == internal::Rule::unsorted_variable => {
                    BasicIntegerTerm::IntegerVariable(pair.as_str().into())
                }
                Some(pair) => Self::report_unexpected_pair(pair),
                None => Self::report_missing_pair(),
            },
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
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct IntegerTermParser;

impl PestParser for IntegerTermParser {
    type Node = IntegerTerm;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::n_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        internal::PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                internal::Rule::n_term => IntegerTermParser::translate_pair(primary),
                internal::Rule::n_basic_term => {
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTermParser::translate_pair(primary))
                }
                _ => Self::report_unexpected_pair(primary),
            })
            .map_prefix(|op, arg| IntegerTerm::UnaryOperation {
                op: UnaryOperatorParser::translate_pair(op),
                arg: Box::new(arg),
            })
            .map_infix(|lhs, op, rhs| IntegerTerm::BinaryOperation {
                op: BinaryOperatorParser::translate_pair(op),
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
            .parse(pair.into_inner())
    }
}

pub struct GeneralTermParser;

impl PestParser for GeneralTermParser {
    type Node = GeneralTerm;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::g_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::g_term => Self::translate_pairs(pair.into_inner()),
            internal::Rule::symbolic_constant => GeneralTerm::Symbol(pair.as_str().into()),
            internal::Rule::n_term => {
                GeneralTerm::IntegerTerm(IntegerTermParser::translate_pair(pair))
            }
            internal::Rule::g_variable => match pair.into_inner().next() {
                Some(pair) if pair.as_rule() == internal::Rule::unsorted_variable => {
                    GeneralTerm::GeneralVariable(pair.as_str().into())
                }
                Some(pair) => Self::report_unexpected_pair(pair),
                None => Self::report_missing_pair(),
            },
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

// TODO Zach: Continue implementing pest parsing for first-order logic here

#[cfg(test)]
mod tests {
    use {
        super::{
            BasicIntegerTermParser, BinaryOperatorParser, IntegerTermParser, UnaryOperatorParser,
        },
        crate::{
            parsing::TestedParser,
            syntax_tree::fol::{BasicIntegerTerm, BinaryOperator, IntegerTerm, UnaryOperator},
        },
    };

    #[test]
    fn parse_unary_operator() {
        UnaryOperatorParser.should_parse_into([("-", UnaryOperator::Negative)]);
    }

    #[test]
    fn parse_binary_operator() {
        BinaryOperatorParser.should_parse_into([
            ("+", BinaryOperator::Add),
            ("-", BinaryOperator::Subtract),
            ("*", BinaryOperator::Multiply),
        ]);
    }

    #[test]
    fn parse_basic_integer_term() {
        BasicIntegerTermParser
            .should_parse_into([
                ("#inf", BasicIntegerTerm::Infimum),
                ("#sup", BasicIntegerTerm::Supremum),
                ("0", BasicIntegerTerm::Numeral(0)),
                ("1", BasicIntegerTerm::Numeral(1)),
                ("-1", BasicIntegerTerm::Numeral(-1)),
                ("-48", BasicIntegerTerm::Numeral(-48)),
                ("301", BasicIntegerTerm::Numeral(301)),
                ("A$i", BasicIntegerTerm::IntegerVariable("A".into())),
            ])
            .should_reject(["00", "-0", "#", "#infi", "#supa", "_", "1_"]);
    }

    #[test]
    fn parse_integer_term() {
        IntegerTermParser
            .should_parse_into([
                (
                    "#inf",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Infimum),
                ),
                (
                    "#sup",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Supremum),
                ),
                (
                    "0",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(0)),
                ),
                (
                    "1",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)),
                ),
                (
                    "-1",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-1)),
                ),
                (
                    "(-48)",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-48)),
                ),
                (
                    "(301)",
                    IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(301)),
                ),
                (
                    "1 + 3 + 2",
                    IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Add,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3)).into(),
                        }
                        .into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2)).into(),
                    },
                ),
            ])
            .should_reject(["00", "#", "#infi", "#supa", "_", "1_", "(1"]);
    }
}
