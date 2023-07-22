pub mod asp;
pub mod fol;

pub trait Formatter {
    type Node: crate::syntax_tree::Node;

    fn format(node: Self::Node) -> String;
}
