#[derive(pest_derive::Parser)]
#[grammar = "parsing/fol.pest"]
pub struct Parser;

#[cfg(test)]
mod tests {
    use super::{Parser, Rule};
    use pest::Parser as _;

    fn recognize(rule: Rule, accept: Vec<&str>, reject: Vec<&str>) {
        for example in accept {
            assert!(
                Parser::parse(rule, example).is_ok(),
                "assertion failed: rule {rule:?} rejects '{example}'"
            );
        }

        for example in reject {
            assert!(
                Parser::parse(rule, example).is_err(),
                "assertion failed: rule {rule:?} accepts '{example}'"
            );
        }
    }

    // TODO Zach: Add tests for the parsing expression grammar here
}
