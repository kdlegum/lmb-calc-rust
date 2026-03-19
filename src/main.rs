use std::{fmt};   

#[derive(Debug)]
enum Element {
    FreeVariable(String),
    Function(Function),
    Application (Application)
}
#[derive(Debug)]
struct Application { on: Box<Element>, from: Box<Element> }
#[derive(Debug)]
struct Function { bound_var: String, body: Box<Element> }

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

fn reduce_single_lambda_function(app: Application) -> Result<Element, String> {
    // This function currently only considers the simple case (λx.{a or x} b) where a and b are simply variables
    let Application { on: func, from: variable} = app;
    let Element::Function(Function {bound_var, body: b}) = *func else {
        return Err("Not a function".to_string());
    };


    let Element::FreeVariable(var_unpacked) = *variable else {
        return Err("Not a variable".to_string());
    };

   let type_b = match *b {
    Element::Application(_) => "app",
    Element::FreeVariable(_) => "var",
    Element::Function(_) => "func"
   };

   if type_b == "var" {

        let Element::FreeVariable(b_unpacked) = *b else {panic!()};

        if bound_var == b_unpacked {
            return Ok(var(&var_unpacked));
        }
        else {
            return Ok(var(&b_unpacked));
        }
   } else if type_b == "func" {
        let Element::Function(mut b_unpacked) = *b else {panic!()};
        let body_of_body = b_unpacked.body;
        let new_body_of_body = body_of_body.to_string().replace(&bound_var, &var_unpacked);
        b_unpacked.body = Box::new(parse_single_element(&new_body_of_body).unwrap());
        return Ok(Element::Function(b_unpacked));
   } else if type_b == "app" {
        let Element::Application(b_unpacked) = *b else {panic!()};
        let Application { on, from } = b_unpacked;
        let new_on_string = replace_excluding_bound_variables(&on.to_string(), var_unpacked.chars().nth(0).unwrap(), bound_var.chars().nth(0).unwrap());
        let new_on = Box::new(parse_single_element(&new_on_string).unwrap());
        let new_from_string = replace_excluding_bound_variables(&from.to_string(), var_unpacked.chars().nth(0).unwrap(), bound_var.chars().nth(0).unwrap());
        let new_from = Box::new(parse_single_element(&new_from_string).unwrap());
        return Ok(Element::Application(Application { on:new_on, from:new_from }));
   };
   Err("unhandled".to_string())
}

fn replace_excluding_bound_variables(s: &str, free_var: char, bound_var: char) -> String {
    // This only works providing the variables are single characters.
    let chars: Vec<char> = s.chars().collect();
    let replaced: String = chars.iter().enumerate().map(|(i, &c)| {
        if c == bound_var && (i == 0 || chars[i-1] != 'λ') {free_var} else {c}
    }).collect();
    replaced
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
                Ok(elem) => return Ok(func(&s.chars().nth(1).unwrap().to_string(), elem)),
                Err(_e) => return Err("Body of function not single".to_string())
            }
        }
        else {
            return Ok(func(&s[2..3], var(&s[4..5])));
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

fn func(s1: &str, elem: Element) -> Element {
    Element::Function(Function { bound_var: s1.to_string(), body: Box::new(elem) })
}

fn identity() -> Element {
    func("x", var("x"))
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
    println!("{}", test);
    let Element::Application(app) = test else {panic!()};
    test = match reduce_single_lambda_function(app) {
        Ok(a) => a,
        Err(e) => panic!()
    };
    println!("{}", test)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Ok cases ---

    // Tests for parse single element
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

    // tests for reduce single lambda function

    #[test]
    fn apply_identity_to_var() {
        let test = apply(identity(), var("a"));
        let Element::Application(app) = test else {panic!()};
        let result = reduce_single_lambda_function(app).unwrap();
        assert_eq!(result.to_string(), "a");
    }

    #[test]
    fn apply_not_identity_to_var() {
        let test = parse_single_element("(λx.y a)").unwrap();
        let Element::Application(app) = test else {panic!()};
        let result = reduce_single_lambda_function(app).unwrap();
        assert_eq!(result.to_string(), "y");
    }

    #[test]
    fn apply_to_application() {
        let test_func = parse_single_element("λx.(b x)").unwrap();
        let Element::Application(test) = apply(test_func, var("a")) else {panic!()};
        let result = reduce_single_lambda_function(test).unwrap();
        assert_eq!(result.to_string(), "(b a)");
    }

    #[test]
    fn apply_to_body_function() {
        let test_body_func = parse_single_element("λy.x").unwrap();
        let test_func = func("x", test_body_func);
        let Element::Application(test) = apply(test_func, var("a")) else {panic!()};
        let result = reduce_single_lambda_function(test).unwrap();
        assert_eq!(result.to_string(), "λy.a");
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
