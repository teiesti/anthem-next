extern crate anthem;

#[test]
fn asp_rule_default_parsing_formatting_identity() {
    for src in [
        "p(X) :- q(X).",
        "p :- not q."
    ] {
        let rule: asp::Rule = src.parse().unwrap();
        let target = format!("{rule}");

        assert_eq!(
            src.to_string(),
            target.to_string(),
            "assertion `left == right` failed:\n left:\n{src}\n right:\n{target}"
        );
    }
}
