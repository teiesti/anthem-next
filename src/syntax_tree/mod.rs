use std::fmt::Debug;

pub mod asp;
pub mod fol;

// TODO: Do we want to specify a default parser (-> FromStr) and formatter (-> Display)?
pub trait Node: Clone + Debug + Eq + PartialEq {}
