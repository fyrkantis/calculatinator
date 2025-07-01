use crate::symbols::exp::Exp;
use crate::discrete::monomial::greatest_common_divisor;

/// (2 * positive - 1) * (numerator/denominator)^power
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
    pub positive: bool,
    pub power: Option<Box<Vec<Fraction>>>
}
impl Fraction {
    pub fn to_string(&self) -> String {
        let sign = match self.positive {true => "", false => "-"};
        let denom = match self.denominator {1 => String::new(), denominator => format!("/{}", denominator)};
        format!("{}{}{}", sign, self.numerator, denom)
    }

    pub fn to_float(&self) -> f64 {
        let mut result = self.numerator as f64;
        if self.denominator != 0 {
            result /= self.denominator as f64;
        }
        if !self.positive {
            result *= -1.;
        }
        /*if root != 1 {
            return f64::powf(result, f64::powi(root, -1));
        }*/
        result
    }
}
impl Default for Fraction {
    fn default() -> Fraction {
        Fraction {
            numerator: 0,
            denominator: 1,
            positive: true,
            power: None
        }
    }
}

pub fn fractinate(expression: &Exp) -> Fraction {
    match expression {
        Exp::Term(a, b) => {
            let (a_frac, b_frac) = (fractinate(a), fractinate(b));
            let gcd = greatest_common_divisor(a_frac.denominator, b_frac.denominator);
            let (a_num, b_num) = (a_frac.numerator * b_frac.denominator / gcd, b_frac.numerator * a_frac.denominator / gcd); // TODO: Make more efficient by looking for common denominators.
            let adding = a_frac.positive == b_frac.positive;
            Fraction {
                numerator: match adding {
                    true => a_num + b_num,
                    false => a_num.abs_diff(b_num)
                },
                denominator: a_frac.denominator * b_frac.denominator / gcd,
                positive: match adding { // TODO: Replace this mess of nested match statements.
                    true => a_frac.positive,
                    false => match a_num > b_num {
                        true => a_frac.positive,
                        false => b_frac.positive
                    }
                },
                ..Default::default()
            }
        },
        Exp::Factor(a, b) => {
            let (a_frac, b_frac) = (fractinate(a), fractinate(b));
            let (numerator, denominator) = (a_frac.numerator * b_frac.numerator, a_frac.denominator * b_frac.denominator);
            let gcd = greatest_common_divisor(numerator, denominator); // TODO: Check if this can be made more efficient. (gcd before multiplication?)
            Fraction {
                numerator: numerator / gcd,
                denominator: denominator / gcd,
                positive: a_frac.positive == b_frac.positive,
                ..Default::default()
            }
        },
        Exp::Pow(a, b) => {
            let (a_frac, b_frac) = (fractinate(a), fractinate(b));
            if b_frac.denominator > 1 {
                println!("WARNING: Raising to the power of a fraction (roots) is not implemented."); // TODO: Implement!
            }
            /*if !a_frac.positive && b_frac.denominator > 1 { // BUG: This will handle 2/2 as a root, simplify first.
                panic!("Imaginary numbers are not implemented. (negative root)"); // TODO: Better error handling.
            }*/ // BUG: Negative odd roots should be possible.
            Fraction {
                numerator: u32::pow(a_frac.numerator, b_frac.numerator), // BUG: Too large numbers *will* crash the program.
                denominator: u32::pow(a_frac.denominator, b_frac.numerator),
                positive: a_frac.positive || b_frac.numerator % 2 == 0,
                ..Default::default()
            }
        },
        Exp::Negative(a) => {
            let frac = fractinate(a);
            Fraction {
                numerator: frac.numerator,
                denominator: frac.denominator,
                positive: !frac.positive,
                ..Default::default()
            }
        },
        Exp::Inverse(a) => {
            let frac = fractinate(a);
            if frac.numerator == 0 {panic!("Division by zero.");} // TODO: Better error handling.
            Fraction {
                numerator: frac.denominator,
                denominator: frac.numerator,
                positive: frac.positive,
                ..Default::default()
            }
        },
        Exp::Number(value) => {
            Fraction {
                numerator: value.digits,
                denominator: 10_u32.pow(value.decimals),
                ..Default::default()
            }
        }
    }
}
