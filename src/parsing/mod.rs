pub mod asp;
pub mod fol;

use pest::{
    error::{Error, ErrorVariant},
    iterators::Pairs,
    Parser, Position, RuleType,
};

pub trait CompleteParser<R: RuleType>: Parser<R> {
    fn parse_complete(rule: R, input: &str) -> Result<Pairs<'_, R>, Error<R>> {
        Self::parse(rule, input).and_then(|pairs| {
            if pairs.as_str() == input {
                Ok(pairs)
            } else {
                Err(Error::new_from_pos(
                    ErrorVariant::CustomError {
                        message: String::from("expected EOI"),
                    },
                    Position::new(input, pairs.as_str().len()).unwrap(),
                ))
            }
        })
    }
}

impl<P: Parser<R>, R: RuleType> CompleteParser<R> for P {}
