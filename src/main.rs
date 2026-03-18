use std::{fmt, str::FromStr};   

#[derive(Debug)]
enum Element {
    FreeVariable(String),
    Function(Function),
    Application (Application)
}
#[derive(Debug)]
struct Application { on: Box<Element>, from: Box<Element> }
#[derive(Debug)]
struct Function { bound_var: String, body: String }

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::FreeVariable(s) => write!(f, "{}", s),
            Element::Function (Function {bound_var: b, body: s}) => write!(f, "λ{}.{}", b, s),
            Element::Application (Application {on: o, from:fr}) => write!(f, "({} {})", o, fr)
        }
    }
}

/* 
Finish this once I have a function that checks if it is done, and a function that simplifies a function or an application by one step.
use std::str::FromStr;
impl FromStr for Element {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.to_string();
        // Find free variables
        let split_expression = x.split_whitespace();
        }
    }
*/

fn reduce_single_lambda_function(s: &str) -> Result<Element, String> {
    todo!("Implement")
}

fn parse_single_element(s: &str) -> Result<Element, String> {
/*
    Conditions for an expression to be a single element in terms of parsing a string.

    If we have an application (on from), on and from must be a single element. We can then return apply(on, from)
    If we have a single function, it's fully simplified (it wasn't wrapped in an application)
    If the first character is not ( nor λ, and it's just a single character ie "x" -> var("x")

    Something like xy is actually (x y) but for now we render this illegal and return Err.

     */
    
    if s.len() == 0 {
        return Err("Empty String".to_string());
    };

    if s.len() == 1 {
        return Ok(var(s));
    };

    let num_lambda = s.matches("λ").count() as u32;
    let num_apps = s.matches("(").count() as u32;

    if num_lambda == 0 && num_apps == 0 {
        return Err("Multiple Elements".to_string());
    }

    if num_lambda > 1 || num_apps > 1 {
        return Err("Multiple Elements".to_string());
    }


    let first_char = s.chars().nth(0).unwrap();
    if first_char == 'λ' {
        let point_index = match s.find(".") {
            Some(v) => v,
            None => return Err("No body in function".to_string())
        };
        if point_index != 3 {
            return Err("Function's bound variable is longer than 1".to_string());
        } else if s[point_index+1..].len() > 1 {
            // Not sure if I need to handle the case λx.(y x)
            let result = parse_single_element(&s[point_index+1..]);
            match result {
                Ok(elem) => return Ok(func(&s.chars().nth(1).unwrap().to_string(), &elem.to_string())),
                Err(e) => return Err("Body of function not single".to_string())
            }
        }
        else {
            return Ok(func(&s[2..3], &s[4..5]));
        };
    };

    if first_char == '(' {
        if s.matches(")").count() as u32 != 1 {
            return Err("Incorrect number of closing brackets".to_string())
        };
        if s[s.len()-1..] != *")" {
            return Err("Invalid expression".to_string())
        };
        let app_content = &s[1..s.len()-1];
        if app_content.matches(" ").count() as u32 != 1 {
            return Err("Invalid application expression".to_string())
        };
        let mut app_content_iter = app_content.split_whitespace();
        let first_elem = app_content_iter.next().unwrap();
        let elem1 = match parse_single_element(&first_elem) {
            Ok(elem) => elem,
            Err(e) => return Err(format!("Error parsing first argument: {}", e))
        };
        let second_elem = app_content_iter.next().unwrap();
        let elem2 = match parse_single_element(&second_elem) {
            Ok(elem) => elem,
            Err(e) => return Err(format!("Error parsing second argument: {}", e))
        };
        return Ok(apply(elem1, elem2));
    }
    Err("Unmatched Error".to_string())
}



fn var(s: &str) -> Element {
    Element::FreeVariable(s.to_string())
}

fn func(s1: &str, s2: &str) -> Element {
    Element::Function(Function { bound_var: s1.to_string(), body: s2.to_string() })
}

fn identity() -> Element {
    func("x", "x")
}

fn apply(elem1: Element, elem2: Element) -> Element {
    Element::Application (Application { on: Box::new(elem1), from: Box::new(elem2) })
}

/*
TODO
i dont think i can implement this yet without being able to make the newly produced string into a element.
fn beta_reduce(func: Function, elem: Element) -> Element {
    let Function {bound_var: bv, body: b};

}
*/

fn main() {
    let mut test = identity();
    test = apply(test, var("y"));
    println!("{}", test)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Ok cases ---

    #[test]
    fn single_variable() {
        let result = parse_single_element("x").unwrap();
        assert_eq!(result.to_string(), "x");
    }

    #[test]
    fn simple_function() {
        let result = parse_single_element("λx.y").unwrap();
        assert_eq!(result.to_string(), "λx.y");
    }

    #[test]
    fn identity_function() {
        let result = parse_single_element("λx.x").unwrap();
        assert_eq!(result.to_string(), "λx.x");
    }

    #[test]
    fn simple_application() {
        let result = parse_single_element("(x y)").unwrap();
        assert_eq!(result.to_string(), "(x y)");
    }

    #[test]
    fn application_with_function() {
        let result = parse_single_element("(λx.y z)").unwrap();
        assert_eq!(result.to_string(), "(λx.y z)");
    }

    #[test]
    fn function_with_application_body() {
        let result = parse_single_element("λx.(y z)").unwrap();
        assert_eq!(result.to_string(), "λx.(y z)");
    }

    // --- Err cases ---

    #[test]
    fn empty_string() {
        assert!(parse_single_element("").is_err());
    }

    #[test]
    fn multiple_bare_chars() {
        assert!(parse_single_element("xy").is_err());
    }

    #[test]
    fn function_no_body() {
        assert!(parse_single_element("λx").is_err());
    }

    #[test]
    fn function_long_bound_var() {
        assert!(parse_single_element("λxy.z").is_err());
    }

    #[test]
    fn application_missing_close_paren() {
        assert!(parse_single_element("(x y").is_err());
    }

    #[test]
    fn application_too_many_spaces() {
        assert!(parse_single_element("(x y z)").is_err());
    }

    #[test]
    fn application_no_space() {
        assert!(parse_single_element("(xy)").is_err());
    }
}
