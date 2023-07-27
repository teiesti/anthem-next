pub mod formatting;
pub mod parsing;
pub mod syntax_tree;

fn main() {
    // TODO DEBUG
    use crate::{
        formatting::asp::default,
        parsing::{asp::pest::ConstantParser, Parser},
    };

    let constant = ConstantParser::parse("1").unwrap();
    println!("{}", default::Format(&constant))
}
