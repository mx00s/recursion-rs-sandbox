use std::ops::{Add, Mul, Sub};

use arith_traits::ICheckedOps;
use recursion::{expand_and_collapse, expand_and_collapse_result, map_layer::MapLayer};

use crate::ast::{Expr, ExprBoxed};

pub fn eval<N>(expr: ExprBoxed<N>) -> N
where
    N: Add<Output = N> + Sub<Output = N> + Mul<Output = N>,
{
    expand_and_collapse(
        expr,
        |boxed| boxed.to_expr(),
        |expr| match expr {
            Expr::Add(a, b) => a + b,
            Expr::Sub(a, b) => a - b,
            Expr::Mul(a, b) => a * b,
            Expr::Literal(a) => a,
        },
    )
}

pub fn try_eval<N>(expr: ExprBoxed<N>) -> Result<N, ()>
where
    N: ICheckedOps<Output = Option<N>>,
{
    expand_and_collapse_result(
        expr,
        |boxed| Ok(boxed.to_expr()),
        |expr: Expr<N, N>| match expr {
            Expr::Add(a, b) => a.checked_add(b).ok_or(()),
            Expr::Sub(a, b) => a.checked_sub(b).ok_or(()),
            Expr::Mul(a, b) => a.checked_mul(b).ok_or(()),
            Expr::Literal(a) => Ok(a),
        },
    )
}

// use recursion::recursive_tree::{ArenaIndex, RecursiveTree};

// impl<'a, A, N, B> MapLayer<B> for &'a Expr<A, N>
// where
//     A: Copy,
// {
//     type To = ExprRef<'a, B, N>;
//     type Unwrapped = A;

//     fn map_layer<F: FnMut(Self::Unwrapped) -> B>(self, mut f: F) -> Self::To {
//         match self {
//             Expr::Add(a, b) => ExprRef::Add(f(*a), f(*b)),
//             Expr::Sub(a, b) => ExprRef::Sub(f(*a), f(*b)),
//             Expr::Mul(a, b) => ExprRef::Mul(f(*a), f(*b)),
//             Expr::Literal(x) => ExprRef::Literal(x),
//         }
//     }
// }

// pub enum ExprRef<'a, A, N> {
//     Add(A, A),
//     Sub(A, A),
//     Mul(A, A),
//     Literal(&'a N),
// }

// type RecursiveExpr<N> = RecursiveTree<Expr<ArenaIndex, N>, ArenaIndex>;

// pub fn eval2<A, N>(expr: &RecursiveExpr<N>) -> N
// where
//     N: Add<Output = N> + Sub<Output = N> + Mul<Output = N>,
// {
//     expand_and_collapse(
//         expr.as_ref(),
//         |x| &x,
//         |expr| match expr {
//             Expr::Add(a, b) => a + b,
//             Expr::Sub(a, b) => a - b,
//             Expr::Mul(a, b) => a * b,
//             Expr::Literal(a) => a,
//         },
//     )
// }
