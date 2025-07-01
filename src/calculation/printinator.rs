use crate::symbols::{exp::Exp, constants::Constant};

pub fn printinate(expression: &Exp, parenthesize: bool) -> String {
    let parenthesis = |input: String| -> String { // TODO: Is this really the prettiest possible solution?
        if parenthesize {
            return format!("({})", input)
        }
        input
    };
    match expression {
        Exp::Term(a, b) => {
            match b.as_ref() {
                Exp::Negative(b_child) => parenthesis(format!("{}-{}", printinate(&a, false), printinate(&b_child, true))),
                Exp::Term(b_child_a, _b_child_b) => {
                    match b_child_a.as_ref() {
                        Exp::Negative(_) => parenthesis(format!("{}{}", printinate(&a, false), printinate(&b, false))),
                        _ => parenthesis(format!("{}+{}", printinate(&a, false), printinate(&b, false)))
                    }
                }
                _ => parenthesis(format!("{}+{}", printinate(&a, false), printinate(&b, false)))
            }
        },
        Exp::Factor(a, b) => {
            match b.as_ref() {
                Exp::Inverse(b_child) => parenthesis(format!("{}/{}", printinate(&a, true), printinate(&b_child, true))),
                _ => format!("{}*{}", printinate(&a, true), printinate(&b, true))
            }
        },
        Exp::Pow(a, b) => parenthesis(format!("{}^{}", printinate(&a, true), printinate(&b, true))),
        Exp::Negative(a) => parenthesis(format!("-{}", printinate(&a, true))),
        Exp::Inverse(a) => format!("1/{}", printinate(&a, true)),
        Exp::Number(value) => format!("{}", value.to_string()),
        Exp::Constant(constant) => match constant {
            Constant::Variable(name) => name.to_string(),
            Constant::Pi => String::from("π"),
            Constant::Tau => String::from("τ"),
            Constant::E => String::from("e"),
            Constant::Euler => String::from("γ"),
            Constant::GoldenRatio => String::from("ϕ"),
            Constant::Imaginary => String::from("i")
        }
    }
}
