#[derive(pest_derive::Parser)]
#[grammar = "parsing/asp.pest"]
pub struct Parser;

#[cfg(test)]
mod tests {
    use {
        super::{Parser, Rule},
        crate::parsing::TestedParser,
    };

    #[test]
    fn recognize_constant_infimum() {
        Parser::test_rule(Rule::infimum).should_accept(["#inf", "#infimum"]);
    }

    #[test]
    fn recognize_constant_integer() {
        Parser::test_rule(Rule::integer)
            .should_accept(["0", "1", "42", "4711", "-1"])
            .should_reject(["a", "A", "4 2", "00", "-0", "--1"]);
    }

    #[test]
    fn recognize_constant_symbol() {
        Parser::test_rule(Rule::symbol)
            .should_accept(["a", "aa", "aA", "_a", "'a", "_'x'_'x'_", "noto"])
            .should_reject(["A", "1", "a a", "a-a", "'", "not"]);
    }

    #[test]
    fn recognize_constant_supremum() {
        Parser::test_rule(Rule::supremum).should_accept(["#sup", "#supremum"]);
    }

    // TODO Tobias: Add tests for the remaining syntax of ASP
}
