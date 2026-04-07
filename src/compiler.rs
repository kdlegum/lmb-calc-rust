use crate::abstract_tree::*;
use crate::lambda::*;
use std::str::FromStr;

fn church_numeral(n: i32) -> Element {
    if n < 0 { panic!("Not supporting negative numbers currently") };
    // λf.λx. f (f (... (f x)...))
    let mut body = var("x");
    for _ in 0..n {
        body = apply(var("f"), body);
    }
    func("f", func("x", body))
}

fn bool_as_lambda(b: bool) -> Element {
    if b {
        func("x", func("y", var("x")))
    } else {
        func("x", func("y", var("y")))
    }
}

fn church_succ() -> Element {
    Element::from_str("λn.λf.λx.(f ((n f) x))").unwrap()
}

fn church_add() -> Element {
    Element::from_str("λm.λn.λf.λx.((m f) ((n f) x))").unwrap()
}

fn ycombinator() -> Element {
    Element::from_str("λf.(λx.(f (x x)) λx.(f (x x)))").unwrap()
}

fn church_pred() -> Element {
    Element::from_str(
        "λn.λf.λx.(((n λp.((λa.λb.λc.((c a) b) λu.λv.u) (((p λu.λv.u) (f (p λu.λv.v))) (p λu.λv.v)))) ((λa.λb.λc.((c a) b) λu.λv.v) x)) λu.λv.v)"
    ).unwrap()
}

fn church_sub() -> Element {
    Element::from_str(
        "λm.λn.((n λn.λf.λx.(((n λp.((λa.λb.λc.((c a) b) λu.λv.u) (((p λu.λv.u) (f (p λu.λv.v))) (p λu.λv.v)))) ((λa.λb.λc.((c a) b) λu.λv.v) x)) λu.λv.v)) m)"
    ).unwrap()
}

fn compile_expression(expr: Expr) -> Element {
    todo!()
    /*
    match expr {
        Number(num) => church_numeral(num)
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn church_zero() {
        assert_eq!(church_numeral(0), Element::from_str("λf.λx.x").unwrap());
    }

    #[test]
    fn church_one() {
        assert_eq!(church_numeral(1), Element::from_str("λf.λx.(f x)").unwrap());
    }

    #[test]
    fn church_two() {
        assert_eq!(church_numeral(2), Element::from_str("λf.λx.(f (f x))").unwrap());
    }

    #[test]
    fn church_three() {
        assert_eq!(church_numeral(3), Element::from_str("λf.λx.(f (f (f x)))").unwrap());
    }

    #[test]
    fn succ_church_one() {
        let expr = apply(church_succ(), church_numeral(1));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(2));
    }

    #[test]
    fn add_two_and_three() {
        let expr = apply(apply(church_add(), church_numeral(2)), church_numeral(3));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(5));
    }

    #[test]
    fn pred_debug_steps() {
        let expr = apply(church_pred(), church_numeral(1));
        for (i, step) in NormalisationSteps::new(expr).enumerate() {
            println!("step {}: {}", i, step);
        }
    }

    #[test]
    fn pred_of_three() {
        let expr = apply(church_pred(), church_numeral(3));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(2));
    }

    #[test]
    fn pred_of_one() {
        let expr = apply(church_pred(), church_numeral(1));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(0));
    }

    #[test]
    fn pred_of_zero_clamps() {
        let expr = apply(church_pred(), church_numeral(0));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(0));
    }

    #[test]
    fn sub_three_minus_two() {
        let expr = apply(apply(church_sub(), church_numeral(3)), church_numeral(2));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(1));
    }

    #[test]
    fn sub_clamps_at_zero() {
        let expr = apply(apply(church_sub(), church_numeral(2)), church_numeral(5));
        let result = normalise_output(expr);
        assert_eq!(result, church_numeral(0));
    }

}