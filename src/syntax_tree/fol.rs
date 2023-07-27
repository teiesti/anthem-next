use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Primitive {
    Infimum,
    Supremum,
}

impl Node for Primitive {}

// TODO Zach: Implement the abstract syntax tree for first-order logic here
