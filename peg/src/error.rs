use crate::BoxedClass;

#[derive(Clone, Debug)]
pub enum Error<T, N> {
    Terminal { expected: BoxedClass<T>, actual: T },
    NonTerminal { unrecognized: N },
    PrematureEndOfInput,
    NotPredicateFail,
}
