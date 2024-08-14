use {
    crate::syntax_tree::asp::{self, Program},
    indexmap::IndexSet,
    petgraph::{algo::is_cyclic_directed, graph::DiGraph},
    std::collections::HashMap,
};

pub trait Tightness {
    fn is_tight(&self) -> bool;

    fn private_recursion_free(&self, private_predicates: IndexSet<asp::Predicate>) -> bool;
}

impl Tightness for Program {
    fn is_tight(&self) -> bool {
        let mut dependency_graph = DiGraph::<(), ()>::new();
        let mut mapping = HashMap::new();

        for predicate in self.predicates() {
            let node = dependency_graph.add_node(());
            mapping.insert(predicate, node);
        }

        for rule in &self.rules {
            if let Some(head_predicate) = rule.head.predicate() {
                for positive_body_predicate in rule.body.positive_predicates() {
                    dependency_graph.update_edge(
                        mapping[&head_predicate],
                        mapping[&positive_body_predicate],
                        (),
                    );
                }
            }
        }

        !is_cyclic_directed(&dependency_graph)
    }

    fn private_recursion_free(&self, private_predicates: IndexSet<asp::Predicate>) -> bool {
        for rule in &self.rules {
            match rule.head.clone() {
                asp::Head::Choice(a) => {
                    if private_predicates.contains(&a.predicate()) {
                        return false;
                    }
                }
                asp::Head::Basic(_) | asp::Head::Falsity => (),
            }
        }

        let mut dependency_graph = DiGraph::<(), ()>::new();
        let mut mapping = HashMap::new();

        for predicate in self.predicates() {
            if private_predicates.contains(&predicate) {
                let node = dependency_graph.add_node(());
                mapping.insert(predicate, node);
            }
        }

        for rule in &self.rules {
            if let Some(head_predicate) = rule.head.predicate() {
                if private_predicates.contains(&head_predicate) {
                    let body_predicates = rule.body.predicates();
                    let private_body_predicates = body_predicates.intersection(&private_predicates);

                    for predicate in private_body_predicates {
                        dependency_graph.update_edge(
                            mapping[&head_predicate],
                            mapping[predicate],
                            (),
                        );
                    }
                }
            }
        }

        !is_cyclic_directed(&dependency_graph)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::Tightness,
        crate::syntax_tree::{asp::Predicate, asp::Program},
        indexmap::IndexSet,
        std::str::FromStr,
    };

    #[test]
    fn test_tightness() {
        for program in [
            "a.",
            "a :- not a.",
            "a :- not b. b :- not a.",
            "p(a) :- p.",
            "p(X) :- not q(X). q(X) :- p(X).",
        ] {
            assert!(Program::from_str(program).unwrap().is_tight())
        }

        for program in [
            "a :- a.",
            "a :- b. b :- a.",
            "p :- q, not r. p :- r. r :- p.",
        ] {
            assert!(!Program::from_str(program).unwrap().is_tight())
        }
    }

    #[test]
    fn test_private_recursion() {
        for program in ["a :- not b. b :- not a.", "a :- p(1). p(X) :- q(X)."] {
            let a: Predicate = "a/0".parse().unwrap();
            let p: Predicate = "p/1".parse().unwrap();
            let private_predicates: IndexSet<Predicate> = IndexSet::from_iter([a, p]);
            assert!(Program::from_str(program)
                .unwrap()
                .private_recursion_free(private_predicates))
        }

        for program in [
            "{a}.",
            "a :- not a.",
            "{p(X)} :- not not p(X). b :- a.",
            "a :- not b. b :- not a.",
        ] {
            let a: Predicate = "a/0".parse().unwrap();
            let b: Predicate = "b/0".parse().unwrap();
            let p: Predicate = "p/1".parse().unwrap();
            let private_predicates: IndexSet<Predicate> = IndexSet::from_iter([a, b, p]);
            assert!(!Program::from_str(program)
                .unwrap()
                .private_recursion_free(private_predicates))
        }
    }
}
