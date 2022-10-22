mod ast;
mod single_pass;

use crate::{
    ast::{Expr, ExprBoxed},
    single_pass::{eval, try_eval},
};

// Notice that if `i64` is changed to `u64` then:
//   - `eval` panics due to underflow
//   - `try_eval` returns an error
fn example() -> ExprBoxed<i64> {
    ExprBoxed::from(Expr::Mul(
        Expr::Literal(1).into(),
        Expr::Sub(Expr::Literal(2).into(), Expr::Literal(3).into()).into(),
    ))
}

fn main() {
    println!("expr: {:#?}", example());
    println!("try result: {:?}", try_eval(example()));
    println!("result: {:?}", eval(example()));
}
