use peg::{BoxedExpr, Grammar};

fn main() {
    let expr = BoxedExpr::sequence(
        BoxedExpr::sequence(BoxedExpr::string("xy".chars()), BoxedExpr::terminal(..)),
        BoxedExpr::end_of_input(),
    );
    let grammar: Grammar<_, usize> = Grammar::new([], expr);
    println!("{}", &grammar);

    let input = ['x', 'y', 'z'];
    println!("input: {:?}", &input);

    let result = grammar.parse(&input);
    println!("result: {:?}", result);
}
