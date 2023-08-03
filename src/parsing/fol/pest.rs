use crate::{
    parsing::PestParser,
    syntax_tree::fol::{
        Atom, AtomicFormula, BasicIntegerTerm, BinaryConnective, BinaryOperator, Comparison,
        GeneralTerm, Guard, IntegerTerm, Quantification, Quantifier, Relation, Sort,
        UnaryConnective, UnaryOperator, Variable,
    },
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
    const RULE: internal::Rule = internal::Rule::basic_integer_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::basic_integer_term => Self::translate_pairs(pair.into_inner()),
            internal::Rule::infimum => BasicIntegerTerm::Infimum,
            internal::Rule::supremum => BasicIntegerTerm::Supremum,
            internal::Rule::numeral => BasicIntegerTerm::Numeral(pair.as_str().parse().unwrap()),
            internal::Rule::integer_variable => match pair.into_inner().next() {
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
    const RULE: internal::Rule = internal::Rule::integer_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        internal::PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                internal::Rule::integer_term => IntegerTermParser::translate_pair(primary),
                internal::Rule::basic_integer_term => {
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
    const RULE: internal::Rule = internal::Rule::general_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::general_term => Self::translate_pairs(pair.into_inner()),
            internal::Rule::symbolic_constant => GeneralTerm::Symbol(pair.as_str().into()),
            internal::Rule::integer_term => {
                GeneralTerm::IntegerTerm(IntegerTermParser::translate_pair(pair))
            }
            internal::Rule::general_variable => match pair.into_inner().next() {
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
        let terms: Vec<_> = pairs.map(GeneralTermParser::translate_pair).collect();

        Atom { predicate, terms }
    }
}

pub struct RelationParser;

impl PestParser for RelationParser {
    type Node = Relation;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::relation;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::relation => Self::translate_pairs(pair.into_inner()),
            internal::Rule::greater_equal => Relation::GreaterEqual,
            internal::Rule::less_equal => Relation::LessEqual,
            internal::Rule::greater => Relation::Greater,
            internal::Rule::less => Relation::Less,
            internal::Rule::equal => Relation::Equal,
            internal::Rule::not_equal => Relation::NotEqual,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct GuardParser;

impl PestParser for GuardParser {
    type Node = Guard;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: Self::Rule = internal::Rule::guard;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        if pair.as_rule() != internal::Rule::guard {
            Self::report_unexpected_pair(pair)
        }

        let mut pairs = pair.into_inner();

        let relation = RelationParser::translate_pair(
            pairs.next().unwrap_or_else(|| Self::report_missing_pair()),
        );
        let term = GeneralTermParser::translate_pair(
            pairs.next().unwrap_or_else(|| Self::report_missing_pair()),
        );

        if let Some(pair) = pairs.next() {
            Self::report_unexpected_pair(pair)
        }

        Guard { relation, term }
    }
}

pub struct ComparisonParser;

impl PestParser for ComparisonParser {
    type Node = Comparison;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: Self::Rule = internal::Rule::comparison;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        if pair.as_rule() != internal::Rule::comparison {
            Self::report_unexpected_pair(pair)
        }

        let mut pairs = pair.into_inner();

        let term = GeneralTermParser::translate_pair(
            pairs.next().unwrap_or_else(|| Self::report_missing_pair()),
        );

        let guards: Vec<_> = pairs.map(GuardParser::translate_pair).collect();

        Comparison { term, guards }
    }
}

pub struct AtomicFormulaParser;

impl PestParser for AtomicFormulaParser {
    type Node = AtomicFormula;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::atomic_formula;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::atomic_formula => {
                AtomicFormulaParser::translate_pairs(pair.into_inner())
            }
            internal::Rule::falsity => AtomicFormula::Falsity,
            internal::Rule::atom => AtomicFormula::Atom(AtomParser::translate_pair(pair)),
            internal::Rule::comparison => {
                AtomicFormula::Comparison(ComparisonParser::translate_pair(pair))
            }
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct QuantifierParser;

impl PestParser for QuantifierParser {
    type Node = Quantifier;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::quantifier;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::quantifier => QuantifierParser::translate_pairs(pair.into_inner()),
            internal::Rule::forall => Quantifier::Forall,
            internal::Rule::exists => Quantifier::Exists,
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
            internal::Rule::variable => VariableParser::translate_pairs(pair.into_inner()),
            internal::Rule::integer_variable => match pair.into_inner().next() {
                Some(pair) if pair.as_rule() == internal::Rule::unsorted_variable => Variable {
                    name: pair.as_str().into(),
                    sort: Sort::Integer,
                },
                Some(pair) => Self::report_unexpected_pair(pair),
                None => Self::report_missing_pair(),
            },
            internal::Rule::general_variable => match pair.into_inner().next() {
                Some(pair) if pair.as_rule() == internal::Rule::unsorted_variable => Variable {
                    name: pair.as_str().into(),
                    sort: Sort::General,
                },
                Some(pair) => Self::report_unexpected_pair(pair),
                None => Self::report_missing_pair(),
            },
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct QuantificationParser;

impl PestParser for QuantificationParser {
    type Node = Quantification;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::quantification;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        if pair.as_rule() != internal::Rule::quantification {
            Self::report_unexpected_pair(pair)
        }

        let mut pairs = pair.into_inner();

        let quantifier = QuantifierParser::translate_pair(
            pairs.next().unwrap_or_else(|| Self::report_missing_pair()),
        );

        let variables: Vec<_> = pairs.map(VariableParser::translate_pair).collect();

        Quantification {
            quantifier,
            variables,
        }
    }
}

pub struct UnaryConnectiveParser;

impl PestParser for UnaryConnectiveParser {
    type Node = UnaryConnective;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::unary_connective;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::negation => UnaryConnective::Negation,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

pub struct BinaryConnectiveParser;

impl PestParser for BinaryConnectiveParser {
    type Node = BinaryConnective;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::binary_connective;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::binary_connective => Self::translate_pairs(pair.into_inner()),
            internal::Rule::equivalence => BinaryConnective::Equivalence,
            internal::Rule::implication => BinaryConnective::Implication,
            internal::Rule::reverse_implication => BinaryConnective::ReverseImplication,
            internal::Rule::conjunction => BinaryConnective::Conjunction,
            internal::Rule::disjunction => BinaryConnective::Disjunction,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

// TODO Zach: Continue implementing pest parsing for first-order logic here

#[cfg(test)]
mod tests {
    use {
        super::{
            AtomParser, AtomicFormulaParser, BasicIntegerTermParser, BinaryConnectiveParser,
            BinaryOperatorParser, ComparisonParser, GeneralTermParser, GuardParser,
            IntegerTermParser, QuantificationParser, QuantifierParser, RelationParser,
            UnaryConnectiveParser, UnaryOperatorParser, VariableParser,
        },
        crate::{
            parsing::TestedParser,
            syntax_tree::fol::{
                Atom, AtomicFormula, BasicIntegerTerm, BinaryConnective, BinaryOperator,
                Comparison, GeneralTerm, Guard, IntegerTerm, Quantification, Quantifier, Relation,
                Sort, UnaryConnective, UnaryOperator, Variable,
            },
        },
    };

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
            .should_reject(["00", "-0", "#", "#infi", "#supa", "_", "1_", "A"]);
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
        ]);
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
                    "-X$i",
                    IntegerTerm::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                            "X".into(),
                        ))
                        .into(),
                    },
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
            .should_reject(["00", "#", "#infi", "#supa", "_", "1_", "(1", "X$", "X"]);
    }

    #[test]
    fn parse_general_term() {
        GeneralTermParser
            .should_parse_into([
                (
                    "#inf",
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Infimum,
                    )),
                ),
                (
                    "#sup",
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Supremum,
                    )),
                ),
                (
                    "1",
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(1),
                    )),
                ),
                (
                    "(1)",
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(1),
                    )),
                ),
                (
                    "-1",
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(-1),
                    )),
                ),
                (
                    "-(1)",
                    GeneralTerm::IntegerTerm(IntegerTerm::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                    }),
                ),
                (
                    "--1",
                    GeneralTerm::IntegerTerm(IntegerTerm::UnaryOperation {
                        op: UnaryOperator::Negative,
                        arg: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-1)).into(),
                    }),
                ),
                (
                    "1 + 2",
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2)).into(),
                    }),
                ),
                ("a", GeneralTerm::Symbol("a".into())),
                ("ca_12", GeneralTerm::Symbol("ca_12".into())),
                ("_b12A", GeneralTerm::Symbol("_b12A".into())),
                ("A", GeneralTerm::GeneralVariable("A".into())),
                (
                    "1 + A$i",
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                            "A".into(),
                        ))
                        .into(),
                    }),
                ),
                (
                    "(1 + Nx$i) * (Y$i - B1$i)",
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Multiply,
                        lhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Add,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                                "Nx".into(),
                            ))
                            .into(),
                        }
                        .into(),
                        rhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Subtract,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                                "Y".into(),
                            ))
                            .into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                                "B1".into(),
                            ))
                            .into(),
                        }
                        .into(),
                    }),
                ),
                (
                    "((1 + 2) - -3) * 4",
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Multiply,
                        lhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Subtract,
                            lhs: IntegerTerm::BinaryOperation {
                                op: BinaryOperator::Add,
                                lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1))
                                    .into(),
                                rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2))
                                    .into(),
                            }
                            .into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-3))
                                .into(),
                        }
                        .into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(4)).into(),
                    }),
                ),
                (
                    "1 + 2 * 3",
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                        rhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Multiply,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2)).into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3)).into(),
                        }
                        .into(),
                    }),
                ),
                (
                    "1 * 2 + 3",
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Multiply,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2)).into(),
                        }
                        .into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3)).into(),
                    }),
                ),
            ])
            .should_reject([
                "(a)",
                "-a",
                "(A)",
                "1 + A",
                "1 + a",
                "1-",
                "1 +",
                "+1",
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
                        terms: vec![GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(1),
                        ))],
                    },
                ),
                (
                    "p(1, 2)",
                    Atom {
                        predicate: "p".into(),
                        terms: vec![
                            GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                                BasicIntegerTerm::Numeral(1),
                            )),
                            GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                                BasicIntegerTerm::Numeral(2),
                            )),
                        ],
                    },
                ),
                (
                    "p(X, a)",
                    Atom {
                        predicate: "p".into(),
                        terms: vec![
                            GeneralTerm::GeneralVariable("X".into()),
                            GeneralTerm::Symbol("a".into()),
                        ],
                    },
                ),
            ])
            .should_reject(["p(1,)", "1", "P", "p("]);
    }

    #[test]
    fn parse_relation() {
        RelationParser
            .should_parse_into([
                ("<", Relation::Less),
                (">", Relation::Greater),
                ("<=", Relation::LessEqual),
                (">=", Relation::GreaterEqual),
                ("!=", Relation::NotEqual),
                ("=", Relation::Equal),
            ])
            .should_reject(["< =", "< =", "less"]);
    }

    #[test]
    fn parse_comparison() {
        ComparisonParser
            .should_parse_into([(
                "p < 5",
                Comparison {
                    term: GeneralTerm::Symbol("p".into()),
                    guards: vec![Guard {
                        relation: Relation::Less,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(5),
                        )),
                    }],
                },
            )])
            .should_reject(["A < B < "]);
    }

    #[test]
    fn parse_atomic_formula() {
        AtomicFormulaParser
            .should_parse_into([
                (
                    "1 = N$g",
                    AtomicFormula::Comparison(Comparison {
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(1),
                        )),
                        guards: vec![Guard {
                            relation: Relation::Equal,
                            term: GeneralTerm::GeneralVariable("N".into()),
                        }],
                    }),
                ),
                (
                    "1 <= N$g > 3 < X$i",
                    AtomicFormula::Comparison(Comparison {
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(1),
                        )),
                        guards: vec![
                            Guard {
                                relation: Relation::LessEqual,
                                term: GeneralTerm::GeneralVariable("N".into()),
                            },
                            Guard {
                                relation: Relation::Greater,
                                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                                    BasicIntegerTerm::Numeral(3),
                                )),
                            },
                            Guard {
                                relation: Relation::Less,
                                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                                    BasicIntegerTerm::IntegerVariable("X".into()),
                                )),
                            },
                        ],
                    }),
                ),
                (
                    "p(N$i, 3*2)",
                    AtomicFormula::Atom(Atom {
                        predicate: "p".into(),
                        terms: vec![
                            GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                                BasicIntegerTerm::IntegerVariable("N".into()),
                            )),
                            GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                                op: BinaryOperator::Multiply,
                                lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3))
                                    .into(),
                                rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2))
                                    .into(),
                            }),
                        ],
                    }),
                ),
            ])
            .should_reject(["p and b"]);
    }

    #[test]
    fn parse_unary_connective() {
        UnaryConnectiveParser
            .should_parse_into([("not", UnaryConnective::Negation)])
            .should_reject(["noto", "not(", "n ot"]);
    }

    #[test]
    fn parse_quantifier() {
        QuantifierParser
            .should_parse_into([
                ("forall", Quantifier::Forall),
                ("exists", Quantifier::Exists),
            ])
            .should_reject(["fora", "exis", "ex ists", "forall1", "exists("]);
    }

    #[test]
    fn parse_variable() {
        VariableParser
            .should_parse_into([
                (
                    "X",
                    Variable {
                        name: "X".into(),
                        sort: Sort::General,
                    },
                ),
                (
                    "G1",
                    Variable {
                        name: "G1".into(),
                        sort: Sort::General,
                    },
                ),
                (
                    "X$i",
                    Variable {
                        name: "X".into(),
                        sort: Sort::Integer,
                    },
                ),
                (
                    "Y$g",
                    Variable {
                        name: "Y".into(),
                        sort: Sort::General,
                    },
                ),
            ])
            .should_reject(["X$k", "X $i", "$i", "$g", "a$g"]);
    }

    #[test]
    fn parse_quantification() {
        QuantificationParser
            .should_parse_into([
                (
                    "exists X",
                    Quantification {
                        quantifier: Quantifier::Exists,
                        variables: vec![Variable {
                            name: "X".into(),
                            sort: Sort::General,
                        }],
                    },
                ),
                (
                    "forall X$i Y Z$g",
                    Quantification {
                        quantifier: Quantifier::Forall,
                        variables: vec![
                            Variable {
                                name: "X".into(),
                                sort: Sort::Integer,
                            },
                            Variable {
                                name: "Y".into(),
                                sort: Sort::General,
                            },
                            Variable {
                                name: "Z".into(),
                                sort: Sort::General,
                            },
                        ],
                    },
                ),
                (
                    "exists G1 G1$i",
                    Quantification {
                        quantifier: Quantifier::Exists,
                        variables: vec![
                            Variable {
                                name: "G1".into(),
                                sort: Sort::General,
                            },
                            Variable {
                                name: "G1".into(),
                                sort: Sort::Integer,
                            },
                        ],
                    },
                ),
            ])
            .should_reject([
                "forall",
                "exists ",
                "exists aA",
                "forall X$k",
                "exists X$i$g",
            ]);
    }
}
