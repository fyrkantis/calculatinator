use crate::symbols::{exp::Exp, constants::Constant};
use std::f64::consts;

pub fn calculatinate(expression: &Exp) -> f64 {
    match expression {
        Exp::Term(a, b) => calculatinate(a) + calculatinate(b),
        Exp::Factor(a, b) => calculatinate(a) * calculatinate(b),
        Exp::Pow(a, b) => f64::powf(calculatinate(a), calculatinate(b)),
        Exp::Negative(a) => -calculatinate(a),
        Exp::Inverse(a) => f64::powi(calculatinate(a), -1),
        Exp::Number(value) => value.to_float(),
        Exp::Constant(constant) => match constant {
            Constant::Variable(_) => 1.,
            Constant::Pi => consts::PI,
            Constant::Tau => consts::TAU,
            Constant::E => consts::E,
            Constant::Euler => 1.,//consts::EGAMMA,
            Constant::GoldenRatio => 1.,//consts::PHI, //TODO: Implement.
            Constant::Imaginary => 1.
        }
    }
}
