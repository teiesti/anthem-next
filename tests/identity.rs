use anthem::syntax_tree::{asp, fol};

#[test]
fn asp_rule_default_parsing_formatting_identity() {
    for src in ["p(X) :- q(X).", "p :- not q."] {
        let rule: asp::Rule = src.parse().unwrap();
        let target = format!("{rule}");

        assert_eq!(
            src.to_string(),
            target.to_string(),
            "assertion `left == right` failed:\n left:\n{src}\n right:\n{target}"
        );
    }
}

#[test]
fn annotated_formula_default_parsing_formatting_identity() {
    for src in [
        "assumption(backward)[p_or_q]: forall X ( p(X) or exists N$ ( q(N$) and N$ = X ) )",
        "spec[covered]: forall Y ( exists X ( covered(Y,X) ) )",
    ] {
        let rule: fol::AnnotatedFormula = src.parse().unwrap();
        let target = format!("{rule}");

        assert_eq!(
            src.to_string(),
            target.to_string(),
            "assertion `left == right` failed:\n left:\n{src}\n right:\n{target}"
        );
    }
}
