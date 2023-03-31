#[derive(pest_derive::Parser)]
#[grammar = "parsing/asp.pest"]
pub struct Parser;

#[cfg(test)]
mod tests {
    use {
        super::{Parser, Rule},
        crate::parsing::CompleteParser as _,
    };

    fn recognize(rule: Rule, accept: Vec<&str>, reject: Vec<&str>) {
        for example in accept {
            assert!(
                Parser::parse_complete(rule, example).is_ok(),
                "assertion failed: rule {rule:?} rejects '{example}'"
            );
        }

        for example in reject {
            assert!(
                Parser::parse_complete(rule, example).is_err(),
                "assertion failed: rule {rule:?} accepts '{example}'"
            );
        }
    }

    #[test]
    fn recognize_infimum() {
        recognize(Rule::infimum, vec!["#inf", "#infimum"], vec![])
    }

    #[test]
    fn recognize_supremum() {
        recognize(Rule::supremum, vec!["#sup", "#supremum"], vec![])
    }

    #[test]
    fn recognize_numeral() {
        recognize(
            Rule::numeral,
            vec!["0", "1", "42", "4711"],
            vec!["a", "A", "4 2", "00"],
        )
    }

    #[test]
    fn recognize_constant() {
        recognize(
            Rule::constant,
            vec!["a", "aa", "aA", "_a", "'a", "_'x'_'x'_", "noto"],
            vec!["A", "1", "a a", "a-a", "'", "not"],
        )
    }

    #[test]
    fn recognize_variable() {
        recognize(
            Rule::variable,
            vec!["A", "AA", "Aa", "_A", "'A", "_'X'_'X'_"],
            vec!["a", "1", "A A", "A-A", "'"],
        )
    }

    #[test]
    fn recognize_term() {
        recognize(
            Rule::term,
            vec![
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
            ],
            vec![
                "1-",
                "1 +",
                "+ 1",
                "1..",
                "..1",
                "(1 + a",
                "1 + a)",
                "(1 (+ a +) 1)",
            ],
        )
    }

    // TODO Tobias: Add tests for the remaining syntax of ASP
}
