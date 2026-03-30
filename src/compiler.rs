use crate::abstract_tree::*;
use crate::lambda::*;
use std::str::FromStr;

fn scott_numeral(n:i32) -> Element {
    if n<0 {panic!("Not supporting negative numbers currently")};
    if n == 0 {
        func("z", func("s", var("z")))
    } else {
        let result = func("z", func("s", apply(var("s"), scott_numeral(n-1))));
        ReductionSteps::new(result).last().unwrap()
    }
}

fn bool_as_lambda(b:bool) -> Element {
    if b {
        func("x", func("y", var("x")))
    } else {
        func("x", func("y", var("y")))
    }
}

fn succ(n:Element) -> Element {
    func("z", func("s", apply(var("s"), n)))
}

fn ycombinator() -> Element {
    Element::from_str("λf.(λx.(f (x x)) λx.(f (x x)))").unwrap()
}

fn compile_expression(expr:Expr) -> Element {
    todo!()
    /*
    match expr {
        Number(num) => scott_numeral(num)
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn church_numeral_two() {
        assert_eq!(scott_numeral(2), Element::from_str("λz.λs.(s λz.λs.(s λz.λs.z))").unwrap())
    }

    #[test]
    fn succ_scott_numeral() {
        println!("{}, {}", scott_numeral(3), succ(scott_numeral(2)));
        assert_eq!(scott_numeral(3), succ(scott_numeral(2)))
    }

}