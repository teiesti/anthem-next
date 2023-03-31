#[derive(pest_derive::Parser)]
#[grammar = "parsing/fol.pest"]
pub struct Parser;

#[cfg(test)]
mod tests {
    // TODO Zach: Add tests for the parsing expression grammar here
}
