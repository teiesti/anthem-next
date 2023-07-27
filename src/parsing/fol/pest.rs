use crate::{parsing::PestParser, syntax_tree::fol::Primitive};

mod internal {
    #[derive(pest_derive::Parser)]
    #[grammar = "parsing/fol/grammar.pest"]
    pub struct Parser;
}

pub struct PrimitiveParser;

impl PestParser for PrimitiveParser {
    type Node = Primitive;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::primitive;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {
            internal::Rule::primitive => Self::translate_pairs(pair.into_inner()),
            internal::Rule::infimum => Primitive::Infimum,
            internal::Rule::supremum => Primitive::Supremum,
            _ => Self::report_unexpected_pair(pair),
        }
    }
}

// TODO Zach: Implement the parsing for first-order logic here
