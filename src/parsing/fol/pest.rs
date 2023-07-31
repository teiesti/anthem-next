use crate::{
    parsing::PestParser,
    syntax_tree::fol::{UnaryOperator, BinaryOperator, BasicIntegerTerm, IntegerTerm, GeneralTerm},
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

pub struct UnaryOperatorParser;

impl PestParser for UnaryOperatorParser {
    type Node = UnaryOperator;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::unary_operator;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            // No need for translate_pairs into_inner since triggering rule is silent
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

pub struct BasicIntegerTermParser;

impl PestParser for BasicIntegerTermParser {
    // Define conversion from PEST pairs to Basic Integer Term type Nodes
    type Node = BasicIntegerTerm;

    type InternalParser = internal::Parser; // Use PEST to produce pairs
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::n_basic_term; // Match n_basic_term in grammar.pest

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::n_basic_term => Self::translate_pairs(pair.into_inner()), // Recurse inward
            internal::Rule::infimum => BasicIntegerTerm::Infimum,
            internal::Rule::supremum => BasicIntegerTerm::Supremum,
            internal::Rule::numeral => BasicIntegerTerm::Numeral(pair.as_str().parse().unwrap()),
            // TODO: Add reference to unsorted variable (if added as node)
            internal::Rule::n_variable => BasicIntegerTerm::IntegerVariable(pair.into_inner().next().unwrap().as_str().into()), // Get variable name
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
        //println!("{pair}\n");
        // The pairs are flattened here (out of necessity) and then PRATT parsing adds the necessary recursion
        // e.g. pratt parsing handles the precedence
        internal::PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                internal::Rule::n_term => IntegerTermParser::translate_pair(primary),
                internal::Rule::n_basic_term => IntegerTerm::BasicIntegerTerm(BasicIntegerTermParser::translate_pair(primary)),
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
            internal::Rule::n_term => GeneralTerm::IntegerTerm(IntegerTermParser::translate_pair(pair)),
            internal::Rule::g_variable => GeneralTerm::GeneralVariable(pair.into_inner().next().unwrap().as_str().into()),
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

// TODO Zach: Continue implementing pest parsing for first-order logic here

#[cfg(test)]
mod tests {
    use {
        super::{
            UnaryOperatorParser, BinaryOperatorParser, BasicIntegerTermParser, IntegerTermParser,
        },
        crate::{
            parsing::TestedParser,
            syntax_tree::fol::{UnaryOperator, BinaryOperator, BasicIntegerTerm, IntegerTerm},
        }
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
            ])
            .should_reject([
                "00", "-0", "#", "#infi", "#supa", "_", "1_"
            ]);
    }

    #[test]
    fn parse_integer_term() {
        IntegerTermParser
            .should_parse_into([
                ("#inf", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Infimum)),
                ("#sup", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Supremum)),
                ("0", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(0))),
                ("1", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1))),
                ("-1", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-1))),
                ("(-48)", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-48))),
                ("(301)", IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(301))),
                (
                    "1 + 3 + 2",
                    IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Add,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3)).into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2)).into(),
                        }
                        .into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                    },
                ),
            ])
            .should_reject([
                "00", "#", "#infi", "#supa", "_", "1_", "(1"
            ]);
    }
}
