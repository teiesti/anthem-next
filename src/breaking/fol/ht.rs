use crate::{
    convenience::unbox::{fol::UnboxedFormula, Unbox},
    syntax_tree::fol::{BinaryConnective, Formula, Quantification, Quantifier, Theory},
};

pub fn break_equivalences(formula: Formula) -> Theory {
    match formula.unbox() {
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Equivalence,
            lhs,
            rhs,
        } => Theory {
            formulas: vec![
                Formula::BinaryFormula {
                    connective: BinaryConnective::Implication,
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(rhs.clone()),
                },
                Formula::BinaryFormula {
                    connective: BinaryConnective::ReverseImplication,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ],
        },
        UnboxedFormula::QuantifiedFormula {
            quantification:
                Quantification {
                    quantifier: Quantifier::Forall,
                    variables,
                },
            formula,
        } => Theory {
            formulas: break_equivalences(formula)
                .formulas
                .into_iter()
                .map(|f| f.quantify(Quantifier::Forall, variables.clone()))
                .collect(),
        },
        x => Theory {
            formulas: vec![x.rebox()],
        },
    }
}

#[cfg(test)]
mod tests {
    use {
        super::break_equivalences,
        crate::syntax_tree::fol::{Formula, Theory},
    };

    #[test]
    fn test_break_equivalences() {
        for (src, target) in [
            ("p <-> q", "p -> q. p <- q."),
            ("p(X) <-> q(X)", "p(X) -> q(X). p(X) <- q(X)."),
            ("forall X (p(X) <-> q(X))", "forall X (p(X) -> q(X)). forall X (p(X) <- q(X))."),
            ("forall X (forall Y (p(X, Y) <-> q(X, Y)))", "forall X (forall Y (p(X, Y) -> q(X, Y))). forall X (forall Y (p(X, Y) <- q(X, Y)))."),
        ] {
            let src: Formula = src.parse().unwrap();
            let target: Theory = target.parse().unwrap();
            assert_eq!(break_equivalences(src), target)
        }
    }
}
