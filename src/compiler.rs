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

fn add_scott(n:Element, m:Element) -> Element {
    // Example (((Y λf.λm.λn.((m n) λp.(succ ((f p) n)))) 2) 1)
    let succ_input = Element::from_str("((f p) n)").unwrap();
    let add = Element::from_str(
        &format!("({} λf.λm.λn.((m n) λp.{}))", ycombinator().to_string(), succ(succ_input).to_string())
    ).unwrap();
    apply(apply(add, n), m)
}

fn determine_scott_numeral(elem: Element) -> i32 {
    // Apply the scott numeral to a known zero marker and identity for succ
    // scott(n) applied to "Z" and (λx.x) gives:
    //   scott(0) Z id = Z
    //   scott(n+1) Z id = id scott(n) = scott(n)
    // So we repeatedly WHNF and count how many times we peel off a successor.
    let zero_marker = var("Q");
    let identity = Element::from_str("λi.i").unwrap();
    let mut current = ReductionSteps::new(elem).last().unwrap();
    current = ReductionSteps::new(apply(current, zero_marker.clone())).last().unwrap();
    current = apply(current, identity.clone());
    let mut count = 0;
    loop {
        current = ReductionSteps::new(current).last().unwrap();
        if count < 10 {
            println!("step {} result: {}", count, current);
        }
        if current == zero_marker {
            return count;
        }
        if count > 100 {
            panic!("determine_scott_numeral exceeded 100 iterations");
        }
        count += 1;
        current = apply(apply(current, zero_marker.clone()), identity.clone());
    }
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

    #[test]
    fn add_scott_one_plus_two() {
        let sum = add_scott(scott_numeral(1), scott_numeral(2));
        assert_eq!(determine_scott_numeral(sum), 3);
    }
}