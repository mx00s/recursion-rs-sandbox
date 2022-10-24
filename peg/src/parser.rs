use crate::class::BoxedClass;

#[derive(Clone, Debug)]
pub enum Error<T, N> {
    Terminal { expected: BoxedClass<T>, actual: T },
    NonTerminal { unrecognized: N },
    PrematureEndOfInput,
    NotPredicateFail,
}
pub struct Parser<'a, T, N>(Box<dyn 'a + Fn(&'a [T]) -> Result<&'a [T], Error<T, N>>>);

impl<'a, T, N: 'a> Parser<'a, T, N> {
    pub fn new(p: Box<dyn 'a + Fn(&'a [T]) -> Result<&'a [T], Error<T, N>>>) -> Self {
        Self(p)
    }

    pub fn parse(&self, input: &'a [T]) -> Result<&'a [T], Error<T, N>> {
        self.0(input)
    }

    pub fn fail(e: Error<T, N>) -> Self
    where
        T: Clone,
        N: Clone,
    {
        Self(Box::new(move |_| Err(e.clone())))
    }

    pub fn empty() -> Self {
        Self(Box::new(|_input| Ok(&[])))
    }

    pub fn and_then(self, next: Self) -> Self {
        Self(Box::new(move |input: &'a [T]| {
            self.parse(input).and_then(|x1| {
                next.parse(&input[x1.len()..])
                    .map(|x2| &input[..(x1.len() + x2.len())])
            })
        }))
    }

    pub fn or_else(self, fallback: Self) -> Self {
        Self(Box::new(move |input| {
            self.parse(input).or_else(|_| fallback.parse(input))
        }))
    }

    pub fn zero_or_more(self) -> Self {
        Self(Box::new(move |input: &'a [T]| {
            self.parse(input).map_or_else(
                |_err| Ok([].as_slice()),
                |x1| {
                    let mut idx = x1.len();
                    while let Ok(xn) = self.parse(&input[idx..]) {
                        idx += xn.len();
                    }
                    Ok(&input[..idx])
                },
            )
        }))
    }

    pub fn negate(self) -> Self {
        Self(Box::new(move |input: &'a [T]| {
            self.parse(input)
                .map_or_else(|_| Ok([].as_slice()), |_| Err(Error::NotPredicateFail))
        }))
    }
}
