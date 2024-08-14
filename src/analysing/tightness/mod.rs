use {
    crate::syntax_tree::asp::Program,
    petgraph::{algo::is_cyclic_directed, graph::DiGraph},
    std::collections::HashMap,
};

pub trait Tightness {
    fn is_tight(&self) -> bool;
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
}

#[cfg(test)]
mod tests {
    use {super::Tightness, crate::syntax_tree::asp::Program, std::str::FromStr};

    #[test]
    fn test_tightness() {
        assert!(!Program::from_str("a :- b. b :- a.").unwrap().is_tight());
        assert!(Program::from_str("a :- not b. b :- not a.")
            .unwrap()
            .is_tight());
    }
}
