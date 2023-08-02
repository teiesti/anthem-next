use crate::{
    parsing::PestParser,
    syntax_tree::asp::{
        Atom, BinaryOperator, Constant, Literal, Sign, Term, UnaryOperator, Variable,
    },
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

pub struct AtomParser;

impl PestParser for AtomParser {
    type Node = Atom;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: Self::Rule = internal::Rule::atom;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        if pair.as_rule() != internal::Rule::atom {
            Self::report_unexpected_pair(pair)
        }

        let mut pairs = pair.into_inner();

        let predicate = pairs
            .next()
            .unwrap_or_else(|| Self::report_missing_pair())
            .as_str()
            .into();
        let terms: Vec<_> = pairs.map(TermParser::translate_pair).collect();

        Atom { predicate, terms }
    }
}

pub struct SignParser;

impl PestParser for SignParser {
    type Node = Sign;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: Self::Rule = internal::Rule::sign;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        if pair.as_rule() != internal::Rule::sign {
            Self::report_unexpected_pair(pair)
        }

        println!("{pair}");

        let mut pairs = pair.into_inner();
        let mut result = Sign::NoSign;

        match pairs.next() {
            None => return result,
            Some(pair) if pair.as_rule() == internal::Rule::negation => {
                result = Sign::Negation;
            }
            Some(pair) => Self::report_unexpected_pair(pair),
        }

        match pairs.next() {
            None => return result,
            Some(pair) if pair.as_rule() == internal::Rule::negation => {
                result = Sign::DoubleNegation;
            }
            Some(pair) => Self::report_unexpected_pair(pair),
        }

        match pairs.next() {
            None => return result,
            Some(pair) => Self::report_unexpected_pair(pair),
        }
    }
}

struct LiteralParser;

impl PestParser for LiteralParser {
    type Node = Literal;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: Self::Rule = internal::Rule::literal;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        if pair.as_rule() != internal::Rule::literal {
            Self::report_unexpected_pair(pair)
        }

        let mut pairs = pair.into_inner();

        let sign =
            SignParser::translate_pair(pairs.next().unwrap_or_else(|| Self::report_missing_pair()));
        let atom =
            AtomParser::translate_pair(pairs.next().unwrap_or_else(|| Self::report_missing_pair()));

        if let Some(pair) = pairs.next() {
            Self::report_unexpected_pair(pair)
        }

        Literal { sign, atom }
    }
}

// TODO Tobias: Continue implementing pest parsing for ASP here

#[cfg(test)]
mod tests {
    use {
        super::{
            AtomParser, BinaryOperatorParser, ConstantParser, LiteralParser, SignParser,
            TermParser, UnaryOperatorParser, VariableParser,
        },
        crate::{
            parsing::TestedParser,
            syntax_tree::asp::{
                Atom, BinaryOperator, Constant, Literal, Sign, Term, UnaryOperator, Variable,
            },
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
            ("/", BinaryOperator::Divide),
            ("\\", BinaryOperator::Modulo),
            ("..", BinaryOperator::Interval),
        ]);
    }

    #[test]
    fn parse_term() {
        TermParser
            .should_parse_into([
                ("#inf", Term::Constant(Constant::Infimum)),
                ("#sup", Term::Constant(Constant::Supremum)),
                ("1", Term::Constant(Constant::Integer(1))),
                ("(1)", Term::Constant(Constant::Integer(1))),
                ("-1", Term::Constant(Constant::Integer(-1))),
                (
                    "-(1)",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::Constant(Constant::Integer(1)).into(),
                    },
                ),
                (
                    "--1",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::Constant(Constant::Integer(-1)).into(),
                    },
                ),
                (
                    "1 + 2",
                    Term::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::Constant(Constant::Integer(2)).into(),
                    },
                ),
                (
                    "1..2",
                    Term::BinaryOperation {
                        op: BinaryOperator::Interval,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::Constant(Constant::Integer(2)).into(),
                    },
                ),
                ("a", Term::Constant(Constant::Symbol("a".into()))),
                ("(a)", Term::Constant(Constant::Symbol("a".into()))),
                (
                    "-a",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::Constant(Constant::Symbol("a".into())).into(),
                    },
                ),
                (
                    "-(a)",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::Constant(Constant::Symbol("a".into())).into(),
                    },
                ),
                (
                    "--a",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::UnaryOperation {
                            op: UnaryOperator::Negative,
                            arg: Term::Constant(Constant::Symbol("a".into())).into(),
                        }
                        .into(),
                    },
                ),
                (
                    "1 + a",
                    Term::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::Constant(Constant::Symbol("a".into())).into(),
                    },
                ),
                (
                    "1..a",
                    Term::BinaryOperation {
                        op: BinaryOperator::Interval,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::Constant(Constant::Symbol("a".into())).into(),
                    },
                ),
                ("A", Term::Variable(Variable::Named("A".into()))),
                ("(A)", Term::Variable(Variable::Named("A".into()))),
                (
                    "-A",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::Variable(Variable::Named("A".into())).into(),
                    },
                ),
                (
                    "-(A)",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::Variable(Variable::Named("A".into())).into(),
                    },
                ),
                (
                    "--A",
                    Term::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: Term::UnaryOperation {
                            op: UnaryOperator::Negative,
                            arg: Term::Variable(Variable::Named("A".into())).into(),
                        }
                        .into(),
                    },
                ),
                (
                    "1 + A",
                    Term::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::Variable(Variable::Named("A".into())).into(),
                    },
                ),
                (
                    "1..A",
                    Term::BinaryOperation {
                        op: BinaryOperator::Interval,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::Variable(Variable::Named("A".into())).into(),
                    },
                ),
                (
                    "(1 + A) * (1 - a)",
                    Term::BinaryOperation {
                        op: BinaryOperator::Multiply,
                        lhs: Term::BinaryOperation {
                            op: BinaryOperator::Add,
                            lhs: Term::Constant(Constant::Integer(1)).into(),
                            rhs: Term::Variable(Variable::Named("A".into())).into(),
                        }
                        .into(),
                        rhs: Term::BinaryOperation {
                            op: BinaryOperator::Subtract,
                            lhs: Term::Constant(Constant::Integer(1)).into(),
                            rhs: Term::Constant(Constant::Symbol("a".into())).into(),
                        }
                        .into(),
                    },
                ),
                (
                    "((1 + 2) - 3) * 4",
                    Term::BinaryOperation {
                        op: BinaryOperator::Multiply,
                        lhs: Term::BinaryOperation {
                            op: BinaryOperator::Subtract,
                            lhs: Term::BinaryOperation {
                                op: BinaryOperator::Add,
                                lhs: Term::Constant(Constant::Integer(1)).into(),
                                rhs: Term::Constant(Constant::Integer(2)).into(),
                            }
                            .into(),
                            rhs: Term::Constant(Constant::Integer(3)).into(),
                        }
                        .into(),
                        rhs: Term::Constant(Constant::Integer(4)).into(),
                    },
                ),
                (
                    "2 * (1..3)",
                    Term::BinaryOperation {
                        op: BinaryOperator::Multiply,
                        lhs: Term::Constant(Constant::Integer(2)).into(),
                        rhs: Term::BinaryOperation {
                            op: BinaryOperator::Interval,
                            lhs: Term::Constant(Constant::Integer(1)).into(),
                            rhs: Term::Constant(Constant::Integer(3)).into(),
                        }
                        .into(),
                    },
                ),
                (
                    "1..3..2",
                    Term::BinaryOperation {
                        op: BinaryOperator::Interval,
                        lhs: Term::BinaryOperation {
                            op: BinaryOperator::Interval,
                            lhs: Term::Constant(Constant::Integer(1)).into(),
                            rhs: Term::Constant(Constant::Integer(3)).into(),
                        }
                        .into(),
                        rhs: Term::Constant(Constant::Integer(2)).into(),
                    },
                ),
                (
                    "1 + 2 * 3",
                    Term::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: Term::Constant(Constant::Integer(1)).into(),
                        rhs: Term::BinaryOperation {
                            op: BinaryOperator::Multiply,
                            lhs: Term::Constant(Constant::Integer(2)).into(),
                            rhs: Term::Constant(Constant::Integer(3)).into(),
                        }
                        .into(),
                    },
                ),
                (
                    "1 * 2 + 3",
                    Term::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: Term::BinaryOperation {
                            op: BinaryOperator::Multiply,
                            lhs: Term::Constant(Constant::Integer(1)).into(),
                            rhs: Term::Constant(Constant::Integer(2)).into(),
                        }
                        .into(),
                        rhs: Term::Constant(Constant::Integer(3)).into(),
                    },
                ),
            ])
            .should_reject([
                "1-",
                "1 +",
                "+ 1",
                "1..",
                "..1",
                "(1 + a",
                "1 + a)",
                "(1 (+ a +) 1)",
            ]);
    }

    #[test]
    fn parse_atom() {
        AtomParser
            .should_parse_into([
                (
                    "p",
                    Atom {
                        predicate: "p".into(),
                        terms: vec![],
                    },
                ),
                (
                    "p()",
                    Atom {
                        predicate: "p".into(),
                        terms: vec![],
                    },
                ),
                (
                    "p(1)",
                    Atom {
                        predicate: "p".into(),
                        terms: vec![Term::Constant(Constant::Integer(1))],
                    },
                ),
                (
                    "p(1, 2)",
                    Atom {
                        predicate: "p".into(),
                        terms: vec![
                            Term::Constant(Constant::Integer(1)),
                            Term::Constant(Constant::Integer(2)),
                        ],
                    },
                ),
            ])
            .should_reject(["p(1,)", "1", "P", "p("]);
    }

    #[test]
    fn parse_sign() {
        SignParser
            .should_parse_into([
                ("", Sign::NoSign),
                ("not", Sign::Negation),
                ("not  not", Sign::DoubleNegation),
            ])
            .should_reject(["notnot", "noto"]);
    }

    #[test]
    fn parse_literal() {
        LiteralParser.should_parse_into([
            (
                "p",
                Literal {
                    sign: Sign::NoSign,
                    atom: Atom {
                        predicate: "p".into(),
                        terms: vec![],
                    },
                },
            ),
            (
                "not p",
                Literal {
                    sign: Sign::Negation,
                    atom: Atom {
                        predicate: "p".into(),
                        terms: vec![],
                    },
                },
            ),
            (
                "not not p",
                Literal {
                    sign: Sign::DoubleNegation,
                    atom: Atom {
                        predicate: "p".into(),
                        terms: vec![],
                    },
                },
            ),
            (
                "notp",
                Literal {
                    sign: Sign::NoSign,
                    atom: Atom {
                        predicate: "notp".into(),
                        terms: vec![],
                    },
                },
            ),
        ]);
    }

    // TODO Tobias: Add tests for the remaining parsers
}
