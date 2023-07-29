use crate::{parsing::PestParser, syntax_tree::fol::BasicIntegerTerm, syntax_tree::fol::IntegerTerm};

mod internal {
    #[derive(pest_derive::Parser)]
    #[grammar = "parsing/fol/grammar.pest"]
    pub struct Parser;
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

pub struct UnaryOperatorParser;

impl PestParser for UnaryOperatorParser {
    type Node = UnaryOperator;

    type InternalParser = internal::Parser;
    type Rule = internal::Rule;
    const RULE: internal::Rule = internal::Rule::n_term;

    fn translate_pair(pair: pest::iterators::Pair<'_, Self::Rule>) -> Self::Node {
        match pair.as_rule() {          // No need for translate_pairs into_inner since triggering rule is silent
            internal::Rule::negative => UnaryOperator::Negative,
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


}

// TODO Zach: Continue implementing pest parsing for first-order logic here

#[cfg(test)]
mod tests {
    use {
        super::PrimitiveParser,
        crate::{parsing::TestedParser, syntax_tree::fol::Primitive},
    };

    #[test]
    fn parse_primitive() {
        PrimitiveParser
            .should_parse_into([
                ("#inf", Primitive::Infimum),
                // ("#infimum", Primitive::Infimum),
                ("#sup", Primitive::Supremum),
                // ("#supremum", Primitive::Supremum),
            ])
            .should_reject([
                // TODO Zach: Add examples
            ]);
    }

    // TODO Zach: Add tests for the remaining parsers
}
