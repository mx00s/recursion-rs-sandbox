use std::{
    fmt,
    ops::{RangeFull, RangeInclusive},
};

use recursion::{expand_and_collapse, map_layer::MapLayer};

use crate::{
    parser::{Error, Parser},
    traits::Bounded,
};

#[derive(Clone, Debug)]
#[allow(unused)]
pub(crate) enum Class<T, R> {
    Range(RangeInclusive<T>),
    Composite(R, R),
}

impl<T, A, B> MapLayer<B> for Class<T, A> {
    type To = Class<T, B>;
    type Unwrapped = A;

    fn map_layer<F: FnMut(Self::Unwrapped) -> B>(self, mut f: F) -> Self::To {
        match self {
            Class::Range(r) => Class::Range(r),
            Class::Composite(c1, c2) => Class::Composite(f(c1), f(c2)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoxedClass<T>(Box<Class<T, Self>>);

impl<T> BoxedClass<T>
where
    T: Clone + Ord,
{
    pub(crate) fn new(c: Class<T, Self>) -> Self {
        Self(Box::new(c))
    }

    fn to_class(self) -> Class<T, Self> {
        *self.0
    }

    pub fn generate_parser<'a, N: 'a>(self) -> Parser<'a, T, N> {
        expand_and_collapse(
            self,
            |boxed: BoxedClass<T>| boxed.to_class(),
            |class: Class<T, Parser<'a, T, N>>| match class {
                Class::Range(r) => Self::range_parser(r),
                Class::Composite(p1, p2) => p1.or_else(p2),
            },
        )
    }

    fn range_parser<'a, N: 'a>(r: RangeInclusive<T>) -> Parser<'a, T, N> {
        Parser::new(Box::new(move |input| {
            input.get(..1).map_or_else(
                || Err(Error::PrematureEndOfInput),
                |front| {
                    if r.contains(&front[0]) {
                        Ok(front)
                    } else {
                        Err(Error::Terminal {
                            actual: front[0].clone(),
                            expected: Self::from(r.clone()),
                        })
                    }
                },
            )
        }))
    }
}

impl<T> From<T> for BoxedClass<T>
where
    T: Clone + fmt::Debug,
{
    fn from(terminal: T) -> Self {
        Self(Box::new(Class::Range(terminal.clone()..=terminal)))
    }
}

impl<T> From<RangeFull> for BoxedClass<T>
where
    T: Bounded,
{
    fn from(_: RangeFull) -> Self {
        Self(Box::new(Class::Range(T::minimum()..=T::maximum())))
    }
}

impl<T> From<RangeInclusive<T>> for BoxedClass<T> {
    fn from(r: RangeInclusive<T>) -> Self {
        Self(Box::new(Class::Range(r)))
    }
}

impl fmt::Display for BoxedClass<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chr = |f: &mut fmt::Formatter<'_>, c: char| {
            if c.is_control() {
                write!(f, "{:?}", c)
            } else {
                write!(f, "{}", c)
            }
        };

        match &*self.0 {
            Class::Range(r) => {
                if r.start() == r.end() {
                    chr(f, *r.start())
                } else {
                    chr(f, *r.start())?;
                    write!(f, "-")?;
                    chr(f, *r.end())
                }
            }
            Class::Composite(c1, c2) => write!(f, "{}{}", c1, c2),
        }
    }
}
