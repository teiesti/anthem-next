pub mod asp;
pub mod fol;

use {
    pest::{
        error::{Error, ErrorVariant},
        iterators::Pairs,
        Parser, Position, RuleType,
    },
    std::marker::PhantomData,
};

pub trait CompleteParser<R: RuleType>: Parser<R> {
    #[allow(clippy::result_large_err)] // to match Parser::parse
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

pub trait TestedParser<R: RuleType>: CompleteParser<R> {
    fn test_rule(rule: R) -> TestedRule<Self, R>
    where
        Self: Sized,
    {
        TestedRule {
            parser: PhantomData,
            rule,
        }
    }
}

impl<P: CompleteParser<R>, R: RuleType> TestedParser<R> for P {}

pub struct TestedRule<P: CompleteParser<R>, R: RuleType> {
    parser: PhantomData<P>,
    rule: R,
}

impl<P: CompleteParser<R>, R: RuleType> TestedRule<P, R> {
    pub fn should_accept<'a>(&self, examples: impl IntoIterator<Item = &'a str>) -> &Self {
        for example in examples {
            assert!(
                P::parse_complete(self.rule, example).is_ok(),
                "assertion failed: rule {:?} rejects '{example}'",
                self.rule
            );
        }
        self
    }

    pub fn should_reject<'a>(&self, examples: impl IntoIterator<Item = &'a str>) -> &Self {
        for example in examples {
            assert!(
                P::parse_complete(self.rule, example).is_err(),
                "assertion failed: rule {:?} accepts '{example}'",
                self.rule
            );
        }
        self
    }
}
