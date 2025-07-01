use crate::symbols::exp::Exp;

pub fn calculatinate(expression: &Exp) -> f64 {
    match expression {
        Exp::Term(a, b) => calculatinate(a) + calculatinate(b),
        Exp::Factor(a, b) => calculatinate(a) * calculatinate(b),
        Exp::Pow(a, b) => f64::powf(calculatinate(a), calculatinate(b)),
        Exp::Negative(a) => -calculatinate(a),
        Exp::Inverse(a) => f64::powi(calculatinate(a), -1),
        Exp::Number(value) => value.to_float()
    }
}
