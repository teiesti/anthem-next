use std::vec;

use crate::{
    convenience::unbox::{fol::UnboxedFormula, Unbox},
    syntax_tree::fol::{self, AnnotatedFormula, Formula},
};

impl Formula {
    // Qx (Head <=> Body)   becomes     [Qx (Head => Body), Qx (Head <= Body)]
    // Head <=> Body        becomes     [Head => Body, Head <= Body]
    pub fn break_equivalences(self) -> Option<Vec<Formula>> {
        match self {
            Formula::QuantifiedFormula {
                quantification: q,
                formula: f,
            } => match f.unbox() {
                UnboxedFormula::BinaryFormula {
                    connective: fol::BinaryConnective::Equivalence,
                    lhs: f1,
                    rhs: f2,
                } => {
                    let imp1 = Formula::QuantifiedFormula {
                        quantification: q.clone(),
                        formula: Formula::BinaryFormula {
                            connective: fol::BinaryConnective::Implication,
                            lhs: f1.clone().into(),
                            rhs: f2.clone().into(),
                        }
                        .into(),
                    };
                    let imp2 = fol::Formula::QuantifiedFormula {
                        quantification: q.clone(),
                        formula: fol::Formula::BinaryFormula {
                            connective: fol::BinaryConnective::ReverseImplication,
                            lhs: f1.into(),
                            rhs: f2.into(),
                        }
                        .into(),
                    };
                    Some(vec![imp1, imp2])
                }
                _ => None,
            },

            fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Equivalence,
                lhs: f1,
                rhs: f2,
            } => {
                let imp1 = fol::Formula::BinaryFormula {
                    connective: fol::BinaryConnective::Implication,
                    lhs: f1.clone(),
                    rhs: f2.clone(),
                };
                let imp2 = fol::Formula::BinaryFormula {
                    connective: fol::BinaryConnective::ReverseImplication,
                    lhs: f1.clone(),
                    rhs: f2.clone(),
                };
                Some(vec![imp1, imp2])
            }

            _ => None,
        }
    }
}

impl AnnotatedFormula {
    // If the internal formula is unbreakable, returns a vector containing itself. Otherwise,
    // (name, role, direction, Qx (Head <=> Body))   becomes
    // [ (name_forward, role, direction, Qx (Head => Body)), (name_backward, role, direction, Qx (Head <= Body)) ]
    // (name, role, direction, Head <=> Body)        becomes
    // [ (name_forward, role, direction, Head => Body), (name_backward, role, direction, Head <= Body) ]
    pub fn break_equivalences(self) -> Vec<AnnotatedFormula> {
        match self.clone().formula.break_equivalences() {
            Some(mut formulas) => {
                let f2 = AnnotatedFormula {
                    role: self.role.clone(),
                    direction: self.direction,
                    name: format!("{}_backward", self.name.clone()),
                    formula: formulas.pop().unwrap(),
                };
                let f1 = AnnotatedFormula {
                    role: self.role,
                    direction: self.direction,
                    name: format!("{}_forward", self.name),
                    formula: formulas.pop().unwrap(),
                };
                vec![f1, f2]
            }
            None => vec![self],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax_tree::fol;

    #[test]
    fn test_break_equivalences_some() {
        for (src, target) in [
            (
                "forall X (p(X) <-> q(X))",
                ["forall X (p(X) -> q(X))", "forall X (p(X) <- q(X))"],
            ),
            (
                "forall X ((p(X) and q) <-> t)",
                [
                    "forall X ((p(X) and q) -> t)",
                    "forall X ((p(X) and q) <- t)",
                ],
            ),
            ("p <-> q", ["p -> q", "p <- q"]),
            ("p <-> q or t", ["p -> q or t", "p <- q or t"]),
            ("p(X) <-> q(X)", ["p(X) -> q(X)", "p(X) <- q(X)"]),
        ] {
            let f: fol::Formula = src.parse().unwrap();
            let result: Option<Vec<fol::Formula>> =
                Some(vec![target[0].parse().unwrap(), target[1].parse().unwrap()]);
            assert_eq!(result, f.break_equivalences())
        }
    }

    #[test]
    fn test_break_equivalences_none() {
        for src in [
            "forall X (p(X) -> q(X))",
            "q(a)",
            "forall X (p(X) and (q <-> t))",
        ] {
            let f: fol::Formula = src.parse().unwrap();
            assert_eq!(None, f.break_equivalences())
        }
    }

    #[test]
    fn test_break_equivalences() {
        for (src, target) in [
            (
                "spec(forward)[p_def]: forall X (p(X) <-> q(X))",
                [
                    "spec(forward)[p_def_forward]: forall X (p(X) -> q(X))",
                    "spec(forward)[p_def_backward]: forall X (p(X) <- q(X))",
                ],
            ),
            (
                "assumption: p <-> q",
                [
                    "assumption(universal)[_forward]: p -> q",
                    "assumption(universal)[_backward]: p <- q",
                ],
            ),
            (
                "lemma[pqt]: forall X (p(X) and (q <-> t))",
                ["lemma[pqt]: forall X (p(X) and (q <-> t))", ""],
            ),
        ] {
            let f: fol::AnnotatedFormula = src.parse().unwrap();
            let mut result: Vec<fol::AnnotatedFormula> = Vec::new();
            for t in target {
                if !t.is_empty() {
                    result.push(t.parse().unwrap());
                }
            }
            assert_eq!(
                result,
                f.clone().break_equivalences(),
                "left != right:\n {:?} \n!=\n {:?}",
                result,
                f.break_equivalences()
            )
        }
    }
}
