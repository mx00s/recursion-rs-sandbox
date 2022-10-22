use recursion::map_layer::MapLayer;

#[derive(Clone, Debug)]
pub enum Expr<A, N> {
    Add(A, A),
    Sub(A, A),
    Mul(A, A),
    Literal(N),
}

impl<A, N, B> MapLayer<B> for Expr<A, N> {
    type To = Expr<B, N>;
    type Unwrapped = A;

    fn map_layer<F: FnMut(Self::Unwrapped) -> B>(self, mut f: F) -> Self::To {
        match self {
            Self::Add(a, b) => Expr::Add(f(a), f(b)),
            Self::Sub(a, b) => Expr::Sub(f(a), f(b)),
            Self::Mul(a, b) => Expr::Mul(f(a), f(b)),
            Self::Literal(x) => Expr::Literal(x),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExprBoxed<N>(Box<Expr<Self, N>>);

impl<N> ExprBoxed<N> {
    pub fn to_expr(self) -> Expr<Self, N> {
        *self.0
    }
}

impl<N> From<Expr<Self, N>> for ExprBoxed<N> {
    fn from(expr: Expr<Self, N>) -> Self {
        Self(Box::new(expr))
    }
}
