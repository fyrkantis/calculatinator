#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Constant {
    Variable(char),
    Pi,
    Tau,
    E,
    Euler,
    GoldenRatio,
    Imaginary
}

pub fn constant_str(constant: &Constant) -> &str {
    match constant {
        Constant::Variable(name) => str::from_utf8(&[*name as u8]).unwrap(), // TODO: Solution that doesn't involve .unwrap().
        Constant::Pi => "π",
        Constant::Tau => "τ",
        Constant::E => "e",
        Constant::Euler => "γ",
        Constant::GoldenRatio => "ϕ",
        Constant::Imaginary => "i"
    }
}
