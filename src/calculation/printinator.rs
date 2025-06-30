
use crate::util::exp::Exp;

pub fn printiate(expression: &Exp, parenthesize: bool) -> String {
    let parenthesis = |input: String| -> String { // TODO: Is this really the prettiest possible solution?
        if parenthesize {
            return format!("({})", input)
        }
        input
    };
    match expression {
        Exp::Term(a, b) => {
            match b.as_ref() {
                Exp::Negative(b_child) => parenthesis(format!("{}-{}", printiate(&a, false), printiate(&b_child, true))),
                Exp::Term(b_child_a, _b_child_b) => {
                    match b_child_a.as_ref() {
                        Exp::Negative(_) => parenthesis(format!("{}{}", printiate(&a, false), printiate(&b, false))),
                        _ => parenthesis(format!("{}+{}", printiate(&a, false), printiate(&b, false)))
                    }
                }
                _ => parenthesis(format!("{}+{}", printiate(&a, false), printiate(&b, false)))
            }
        },
        Exp::Factor(a, b) => {
            match b.as_ref() {
                Exp::Inverse(b_child) => parenthesis(format!("{}/{}", printiate(&a, true), printiate(&b_child, true))),
                _ => format!("{}*{}", printiate(&a, true), printiate(&b, true))
            }
        },
        Exp::Pow(a, b) => parenthesis(format!("{}^{}", printiate(&a, true), printiate(&b, true))),
        Exp::Negative(a) => parenthesis(format!("-{}", printiate(&a, true))),
        Exp::Inverse(a) => format!("1/{}", printiate(&a, true)),
        Exp::Number(value) => format!("{}", value.to_string())
    }
}
