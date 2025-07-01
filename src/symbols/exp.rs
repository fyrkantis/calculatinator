use crate::symbols::{fpnum, constants};

pub enum Exp {
    /// a + b
    Term(Box<Exp>, Box<Exp>),
    /// a * b
    Factor(Box<Exp>, Box<Exp>),
    /// a^b
    Pow(Box<Exp>, Box<Exp>),
    /// -a
    Negative(Box<Exp>),
    /// 1/a
    Inverse(Box<Exp>),
    Number(fpnum::FixedPointNumber),
//    Constant(constants::Constant)
}
