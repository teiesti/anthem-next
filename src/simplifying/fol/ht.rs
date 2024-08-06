use crate::{
    convenience::{
        apply::Apply as _,
        unbox::{fol::UnboxedFormula, Unbox as _},
    },
    syntax_tree::fol::{AtomicFormula, BinaryConnective, Formula, Theory},
};

pub fn simplify(theory: Theory) -> Theory {
    Theory {
        formulas: theory.formulas.into_iter().map(simplify_formula).collect(),
    }
}

fn simplify_formula(formula: Formula) -> Formula {
    formula.apply_all(&mut vec![
        Box::new(remove_identities),
        Box::new(remove_annihilations),
        Box::new(remove_idempotences),
    ])
}

fn remove_identities(formula: Formula) -> Formula {
    // Remove identities
    // e.g. F op E => F

    match formula.unbox() {
        // F and #true => F
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Conjunction,
            lhs,
            rhs: Formula::AtomicFormula(AtomicFormula::Truth),
        } => lhs,

        // #true and F => F
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Conjunction,
            lhs: Formula::AtomicFormula(AtomicFormula::Truth),
            rhs,
        } => rhs,

        // F or #false => F
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Disjunction,
            lhs,
            rhs: Formula::AtomicFormula(AtomicFormula::Falsity),
        } => lhs,

        // #false or F => F
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Disjunction,
            lhs: Formula::AtomicFormula(AtomicFormula::Falsity),
            rhs,
        } => rhs,

        x => x.rebox(),
    }
}

fn remove_annihilations(formula: Formula) -> Formula {
    // Remove annihilations
    // e.g. F op E => E

    match formula.unbox() {
        // F or #true => #true
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Disjunction,
            lhs: _,
            rhs: rhs @ Formula::AtomicFormula(AtomicFormula::Truth),
        } => rhs,

        // #true or F => #true
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Disjunction,
            lhs: lhs @ Formula::AtomicFormula(AtomicFormula::Truth),
            rhs: _,
        } => lhs,

        // F and #false => false
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Conjunction,
            lhs: _,
            rhs: rhs @ Formula::AtomicFormula(AtomicFormula::Falsity),
        } => rhs,

        // #false and F => #false
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Conjunction,
            lhs: lhs @ Formula::AtomicFormula(AtomicFormula::Falsity),
            rhs: _,
        } => lhs,

        x => x.rebox(),
    }
}

fn remove_idempotences(formula: Formula) -> Formula {
    // Remove idempotences
    // e.g. F op F => F

    match formula.unbox() {
        // F and F => F
        // F or  F => F
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Conjunction | BinaryConnective::Disjunction,
            lhs,
            rhs,
        } if lhs == rhs => lhs,

        x => x.rebox(),
    }
}

#[cfg(test)]
mod tests {
    use super::simplify_formula;

    #[test]
    fn test_simplify_formula() {
        for (src, target) in [
            ("#true and a", "a"),
            ("a and #true", "a"),
            ("#false or a", "a"),
            ("a or #false", "a"),
            ("#true or a", "#true"),
            ("a or #true", "#true"),
            ("#false and a", "#false"),
            ("a and #false", "#false"),
            ("a and a", "a"),
            ("a or a", "a"),
            ("#true and #true and a", "a"),
            ("#true and (#true and a)", "a"),
        ] {
            assert_eq!(
                simplify_formula(src.parse().unwrap()),
                target.parse().unwrap()
            )
        }
    }
}
