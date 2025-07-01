use std::f64::consts;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Constant {
    //Variable(char),
    Pi,
    Tau,
    E,
    Euler,
    GoldenRatio,
    Imaginary
}

pub fn constant_str(constant: &Constant) -> &str {
    match constant {
        //Constant::Variable(name) => str::from_utf8(&[*name as u8]).unwrap(), // TODO: Solution that doesn't involve .unwrap().
        Constant::Pi => "π",
        Constant::Tau => "τ",
        Constant::E => "e",
        Constant::Euler => "γ",
        Constant::GoldenRatio => "ϕ",
        Constant::Imaginary => "i"
    }
}

pub fn constant_float(constant: &Constant) -> f64 {
    match constant {
        //Constant::Variable(_) => 1.,
        Constant::Pi => consts::PI,
        Constant::Tau => consts::TAU,
        Constant::E => consts::E,
        Constant::Euler => 1.,//consts::EGAMMA,
        Constant::GoldenRatio => 1.,//consts::PHI, //TODO: Implement.
        Constant::Imaginary => 1.
    }
}

