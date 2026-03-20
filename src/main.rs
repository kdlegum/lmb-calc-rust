use std::{fmt};   

#[derive(Debug, PartialEq)]
enum Element {
    FreeVariable(String),
    Function(Function),
    Application (Application)
}
#[derive(Debug, PartialEq)]
struct Application { on: Box<Element>, from: Box<Element> }
#[derive(Debug, PartialEq)]
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

use std::str::FromStr;
impl FromStr for Element {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        //println!("{}", s);

        if s.matches("(").count() != s.matches(")").count() {
            return Err("Unclosed/unopened application".to_string());
        };

        let _attempt_parse = match parse_single_element(s) {
            Ok(a) => return Ok(a),
            Err(e) => e
        };

        let first_char: char = match s.to_string().chars().nth(0) {
            Some(a) => a,
            None => return Err("Blank".to_string())
        };

        if first_char == 'λ' {
            let bound_var: char = match s.to_string().chars().nth(1) {
                Some(a) => a,
                None => return Err("No variable after the lambda".to_string())
            };
            let _third_char = match s.to_string().chars().nth(2) {
                Some('.') => "ok",
                Some(_a) => return Err("Only one letter variables allowed".to_string()),
                None => return Err("No body in a function".to_string())
            };
            let result: Element = match Self::from_str(&s[4..]) {
                Ok(a) => a,
                Err(e) => return Err(format!("{}", e))
            };
            return Ok(func(&bound_var.to_string(), result));
        } else if first_char == '(' {
            if s.to_string().chars().last().unwrap() != ')' {
                return Err("Invalid expression".to_string());
            };
            
            let mut bracket_index = 0;
            let mut found_candidate = false;
            let mut candidate_index: usize = 0;
            for (i, c) in s[1..s.len()-1].chars().enumerate() {
                if bracket_index < 0 {return Err("Invalid brackets".to_string())};
                if c == '(' {bracket_index += 1}
                else if c == ')' {bracket_index -= 1} 
                else if c == ' ' && bracket_index == 0 {
                    if found_candidate {return Err("Three or more arguments in application".to_string())}
                    else {
                        found_candidate = true;
                        candidate_index = i;
                    }
                }
            }

            if !found_candidate {return Err("only one argument in application".to_string())};
            
            let chars: Vec<char> = s[1..s.len()-1].chars().collect();
            let str_elem1: String = chars.get(..candidate_index).unwrap_or_default().iter().collect();
            let str_elem2: String = chars.get(candidate_index+1..chars.len()).unwrap_or_default().iter().collect();

            if str_elem1 == "" || str_elem2 == "" {return Err("Application needs two valid arguments".to_string())};

            let elem1: Element = match Self::from_str(&str_elem1) {
                Ok(a) => a,
                Err(e) => return Err(format!("{}", e)),
            };

            let elem2 = match Self::from_str(&str_elem2) {
                Ok(a) => a,
                Err(e) => return Err(format!("{}", e)),
            };

            Ok(apply(elem1, elem2))
            
        } else {
            if s.chars().count() != 1 { return Err("Only single character variables are allowed".to_string()) }
            else {Ok(var(s))}
        }

        }

    }

fn get_element_type(elem: &Element) -> &str {
    match elem {
        Element::Application(_) => "app",
        Element::FreeVariable(_) => "var",
        Element::Function(_) => "func",
    }
}

fn beta_reduce(app: Application) -> Result<Element, String> {
    let Application { on, from } = app;

    if get_element_type(&on) != "func" {
        return Err("Not a function".to_string());
    };

    let Element::Function(Function { bound_var, body }) = *on else {panic!()};

    let body_as_str = body.to_string();
    let from_as_str = from.to_string();
    let new_body = replace_excluding_bound_variables(&body_as_str, &from_as_str, &bound_var);
    let new_body_elem = match Element::from_str(&new_body) {
        Ok(a) => a,
        Err(_) => return Err("Could not parse body".to_string()),
    };
    return Ok(new_body_elem);

}

fn replace_excluding_bound_variables(s: &str, to: &str, bound_var: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut remaining = s;

    while let Some(idx) = remaining.find(bound_var) {
        result.push_str(&remaining[..idx]);

        let is_bound = result.ends_with('λ');

        if is_bound {
            result.push_str(bound_var);
        } else {
            result.push_str(to);
        }

        remaining = &remaining[idx + bound_var.len()..];
    }

    result.push_str(remaining);
    result
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



fn reduce_expression(elem: Element) -> Result<(Element, bool), String> {

    match elem {
        Element::Application(app) => {
            if get_element_type(&app.on) == "func" {
            return Ok((beta_reduce(app).unwrap(), true));
            } else {
                let Application { on, from } = app;
                match reduce_expression(*on) {
                    Ok((a, true)) => return Ok((Element::Application(Application { on:Box::new(a), from }), true)),
                    Ok((elem1, false)) => {
                         match reduce_expression(*from) {
                            Ok((a, b)) => return Ok(((apply(elem1, a)), b)),
                            Err(e) => return Err(e),
                         }
                    },
                    Err(e) => return Err(e),
                };
             }
        },
        Element::Function(func) => {
            match reduce_expression(*func.body) {
                Ok((reduced_body, true)) => return Ok((Element::Function(Function { body: Box::new(reduced_body), ..func }), true)),
                Ok((body, false)) => return Ok((Element::Function(Function { body:Box::new(body), ..func }), false)),
                Err(e) => return Err(e),
            }
        },
        Element::FreeVariable(_) => return Ok((elem, false)),
    }
    
    
    if get_element_type(&elem) != "app" {
        return Ok((elem, false));
    }

    let Element::Application(app) = elem else { panic!() };

    
}

fn main() {
    
    let input = input("Enter a lambda function: ");
    let mut elem = match Element::from_str(&input) {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };
    
    loop {
        match reduce_expression(elem) {
            Ok((a, true)) => {
                elem = a;
                println!("{}", elem);
            },
            Ok((a, false)) => {
                elem = a;
                break;
            },
            Err(e) => panic!("{}", e),
        }
    }
    
    println!("Done. Result: {}", elem.to_string());
    return;
}

use std::io::{self, Write};
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // --- Ok cases ---
    
    // Tests for parse single element
    #[test]
    fn single_variable() {
        let result = Element::from_str("x").unwrap();
        assert_eq!(result.to_string(), "x");
    }

    #[test]
    fn simple_function() {
        let result = Element::from_str("λx.y").unwrap();
        assert_eq!(result.to_string(), "λx.y");
    }

    #[test]
    fn identity_function() {
        let result = Element::from_str("λx.x").unwrap();
        assert_eq!(result.to_string(), "λx.x");
    }

    #[test]
    fn simple_application() {
        let result = Element::from_str("(x y)").unwrap();
        assert_eq!(result.to_string(), "(x y)");
    }

    #[test]
    fn application_with_function() {
        let result = Element::from_str("(λx.y z)").unwrap();
        assert_eq!(result.to_string(), "(λx.y z)");
    }

    #[test]
    fn function_with_application_body() {
        let result = Element::from_str("λx.(y z)").unwrap();
        assert_eq!(result.to_string(), "λx.(y z)");
    }

    // tests for reduce single lambda function

    #[test]
    fn apply_identity_to_var() {
        let test = apply(identity(), var("a"));
        let Element::Application(app) = test else {panic!()};
        let result = beta_reduce(app).unwrap();
        assert_eq!(result.to_string(), "a");
    }

    #[test]
    fn apply_not_identity_to_var() {
        let test = Element::from_str("(λx.y a)").unwrap();
        let Element::Application(app) = test else {panic!()};
        let result = beta_reduce(app).unwrap();
        assert_eq!(result.to_string(), "y");
    }

    #[test]
    fn apply_to_application() {
        let test_func = Element::from_str("λx.(b x)").unwrap();
        let Element::Application(test) = apply(test_func, var("a")) else {panic!()};
        let result = beta_reduce(test).unwrap();
        assert_eq!(result.to_string(), "(b a)");
    }

    #[test]
    fn apply_to_body_function() {
        let test_body_func = Element::from_str("λy.x").unwrap();
        let test_func = func("x", test_body_func);
        let Element::Application(test) = apply(test_func, var("a")) else {panic!()};
        let result = beta_reduce(test).unwrap();
        assert_eq!(result.to_string(), "λy.a");
    }

    #[test]
    fn complex_application() {
        let Element::Application(test) = Element::from_str("(λx.λy.(x y) a)").unwrap() else {panic!()};
        let result = beta_reduce(test).unwrap();
        assert_eq!(result.to_string(), "λy.(a y)");
    }

    // Tests for fromStr for element
    #[test]
    fn nested_functions() {
        let inner_func = Element::from_str("λy.x").unwrap();
        let expected = func("x", inner_func);
        assert_eq!(Element::from_str("λx.λy.x").unwrap(), expected);
    }

    #[test]
    fn nested_applications() {
        let inner_app = Element::from_str("(a b)").unwrap();
        let expected = apply(var("c"), inner_app);
        assert_eq!(Element::from_str("(c (a b))").unwrap(), expected);
    }

    #[test]
    fn complex_lambda_function() {
        let inner_func = Element::from_str("λy.x").unwrap();
        let master_func = func("x", inner_func);
        let inner_app = apply(master_func, var("a"));
        let expected = apply(inner_app, var("b"));
        assert_eq!(Element::from_str("((λx.λy.x a) b)").unwrap(), expected);
    }

    // --- Err cases ---

    #[test]
    fn empty_string() {
        assert!(Element::from_str("").is_err());
    }

    #[test]
    fn multiple_bare_chars() {
        assert!(Element::from_str("xy").is_err());
    }

    #[test]
    fn function_no_body() {
        assert!(Element::from_str("λx").is_err());
    }

    #[test]
    fn function_long_bound_var() {
        assert!(Element::from_str("λxy.z").is_err());
    }

    #[test]
    fn application_missing_close_paren() {
        assert!(Element::from_str("(x y").is_err());
    }

    #[test]
    fn application_too_many_spaces() {
        assert!(Element::from_str("(x y z)").is_err());
    }

    #[test]
    fn application_no_space() {
        assert!(Element::from_str("(xy)").is_err());
    }

    #[test]
    fn space_before_application() {
        assert!(Element::from_str(" (a b)").is_err());
    }

    #[test]
    fn three_argument_application() {
        assert!(Element::from_str("(a b c)").is_err());
    }
}
