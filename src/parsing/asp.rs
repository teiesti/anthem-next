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
    fn recognize_infimum() {
        Parser::test_rule(Rule::infimum).should_accept(["#inf", "#infimum"]);
    }

    #[test]
    fn recognize_supremum() {
        Parser::test_rule(Rule::supremum).should_accept(["#sup", "#supremum"]);
    }

    #[test]
    fn recognize_numeral() {
        Parser::test_rule(Rule::numeral)
            .should_accept(["0", "1", "42", "4711"])
            .should_reject(["a", "A", "4 2", "00"]);
    }

    #[test]
    fn recognize_constant() {
        Parser::test_rule(Rule::constant)
            .should_accept(["a", "aa", "aA", "_a", "'a", "_'x'_'x'_", "noto"])
            .should_reject(["A", "1", "a a", "a-a", "'", "not"]);
    }

    #[test]
    fn recognize_variable() {
        Parser::test_rule(Rule::variable)
            .should_accept(["A", "AA", "Aa", "_A", "'A", "_'X'_'X'_"])
            .should_reject(["a", "1", "A A", "A-A", "'"]);
    }

    #[test]
    fn recognize_term() {
        Parser::test_rule(Rule::term)
            .should_accept([
                "#inf",
                "1",
                "a",
                "A",
                "-1",
                "-a",
                "-A",
                "1 + 1",
                "1 + a",
                "1 + A",
                "1..1",
                "1..a",
                "1..A",
                "--1",
                "(1)",
                "(a)",
                "(A)",
                "(1 + A) * (1 - a)",
                "((1 + 2) - 3) * 4",
                "2 * (1..3)",
                "1..3..2",
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

    // TODO Tobias: Add tests for the remaining syntax of ASP
}
