use std::{collections::HashMap, fmt, iter::Step};

use recursion::{expand_and_collapse, map_layer::MapLayer};

use crate::{Bounded, BoxedClass, Class, Error, Parser};

#[derive(Clone, Debug)]
#[allow(unused)]
pub(crate) enum Expr<T, N, R> {
    Empty,
    Terminal(BoxedClass<T>),
    NonTerminal(N),
    Sequence(R, R),
    Choice(R, R),
    ZeroOrMore(R),
    Not(R),
}

impl<T, N, A, B> MapLayer<B> for Expr<T, N, A> {
    type To = Expr<T, N, B>;
    type Unwrapped = A;

    fn map_layer<F: FnMut(Self::Unwrapped) -> B>(self, mut f: F) -> Self::To {
        match self {
            Expr::Empty => Expr::Empty,
            Expr::Terminal(t) => Expr::Terminal(t),
            Expr::NonTerminal(n) => Expr::NonTerminal(n),
            Expr::Sequence(e1, e2) => Expr::Sequence(f(e1), f(e2)),
            Expr::Choice(e1, e2) => Expr::Choice(f(e1), f(e2)),
            Expr::ZeroOrMore(e) => Expr::ZeroOrMore(f(e)),
            Expr::Not(e) => Expr::Not(f(e)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoxedExpr<T, N>(Box<Expr<T, N, Self>>);

#[allow(unused)]
impl<T, N> BoxedExpr<T, N>
where
    T: Bounded + Clone + fmt::Debug + Eq + Ord + Step,
{
    pub fn empty() -> Self {
        Expr::Empty.into()
    }

    pub fn terminal(class: impl Into<BoxedClass<T>>) -> Self {
        Expr::Terminal(class.into()).into()
    }

    pub fn string(mut ts: impl DoubleEndedIterator<Item = T>) -> Self {
        ts.next_back().map_or_else(
            || Self::empty(),
            |t| {
                ts.rfold(Self::terminal(t), |expr, t| {
                    Self::sequence(Self::terminal(t), expr)
                })
            },
        )
    }

    pub fn any() -> Self
    where
        T: Bounded,
    {
        Self::terminal(BoxedClass::new(Class::Range(T::minimum()..=T::maximum())))
    }

    pub fn non_terminal(n: N) -> Self {
        Expr::NonTerminal(n).into()
    }

    pub fn sequence(e1: Self, e2: Self) -> Self {
        Expr::Sequence(e1, e2).into()
    }

    pub fn choice(e1: Self, e2: Self) -> Self {
        Expr::Choice(e1, e2).into()
    }

    pub fn optional(e: Self) -> Self {
        Self::choice(e, Self::empty())
    }

    pub fn zero_or_more(e: Self) -> Self {
        Expr::ZeroOrMore(e).into()
    }

    pub fn one_or_more(e: Self) -> Self
    where
        T: Clone,
        N: Clone,
    {
        Self::sequence(e.clone(), Self::zero_or_more(e))
    }

    pub fn and(e: Self) -> Self {
        Self::not(Self::not(e))
    }

    pub fn not(e: Self) -> Self {
        Expr::Not(e).into()
    }

    pub fn end_of_input() -> Self {
        Self::not(Self::any())
    }
}

impl<T, N> From<Expr<T, N, Self>> for BoxedExpr<T, N> {
    fn from(e: Expr<T, N, Self>) -> Self {
        Self(Box::new(e))
    }
}

impl<N> fmt::Display for BoxedExpr<char, N>
where
    N: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.0 {
            Expr::Empty => write!(f, "ε"),
            Expr::Terminal(class) => write!(f, "[{}]", class),
            Expr::NonTerminal(n) => write!(f, "rule({})", n),
            Expr::Sequence(e1, e2) => write!(f, "{} {}", e1, e2),
            Expr::Choice(e1, e2) => write!(f, "{} / {}", e1, e2),
            Expr::ZeroOrMore(e) => write!(f, "{}*", e),
            Expr::Not(e) => write!(f, "!{}", e),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grammar<T, N> {
    rules: HashMap<N, BoxedExpr<T, N>>,
    expr: BoxedExpr<T, N>,
}

impl<T, N> Grammar<T, N> {
    pub fn new(
        rules: impl IntoIterator<Item = (N, BoxedExpr<T, N>)>,
        expr: impl Into<BoxedExpr<T, N>>,
    ) -> Self
    where
        N: Eq + std::hash::Hash,
    {
        Self {
            rules: rules.into_iter().collect(),
            expr: expr.into(),
        }
    }

    pub fn parse<'a>(self, input: &'a [T]) -> Result<&'a [T], Error<T, N>>
    where
        T: Bounded + Clone + fmt::Debug + Eq + Ord + Step,
        N: 'a + Clone + Eq + std::hash::Hash,
    {
        Self::generate_parser(&self.rules, self.expr).parse(input)
    }

    fn generate_parser<'a>(
        rules: &HashMap<N, BoxedExpr<T, N>>,
        expr: BoxedExpr<T, N>,
    ) -> Parser<'a, T, N>
    where
        T: Bounded + Clone + fmt::Debug + Eq + Ord + Step,
        N: 'a + Clone + Eq + std::hash::Hash,
    {
        expand_and_collapse(
            expr,
            |boxed: BoxedExpr<T, N>| *boxed.0,
            |expr: Expr<T, N, Parser<'a, T, N>>| match expr {
                Expr::Empty => Parser::empty(),
                Expr::Terminal(t) => t.generate_parser(),
                Expr::NonTerminal(n) => rules.get(&n).map_or_else(
                    || Parser::fail(Error::NonTerminal { unrecognized: n }),
                    |expr| Self::generate_parser(rules, expr.clone()),
                ),
                Expr::Sequence(p1, p2) => p1.and_then(p2),
                Expr::Choice(p1, p2) => p1.or_else(p2),
                Expr::ZeroOrMore(p) => p.zero_or_more(),
                Expr::Not(p) => p.negate(),
            },
        )
    }
}

impl<N> fmt::Display for Grammar<char, N>
where
    N: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PEG:")?;
        writeln!(f, "  rules:")?;
        for (identifier, expr) in &self.rules {
            writeln!(f, "    {} <- {}", identifier, expr)?;
        }
        writeln!(f, "  start: {}", self.expr)
    }
}
