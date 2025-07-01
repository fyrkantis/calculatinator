use std::collections::HashMap;
use crate::symbols::{exp::Exp, constants::{Constant, constant_str, constant_float}};
use crate::discrete::monomial::greatest_common_divisor;



/// (2 * positive - 1) * (numerator/denominator)^power
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
    pub consts: HashMap<Constant, i32>,
    pub positive: bool,
    pub power: Option<Box<Vec<Fraction>>>
}
impl Fraction {
    pub fn to_string(&self) -> String { // TODO: Switch to unsigned integer type, with abs outside the function.
        fn join_consts_str<'a>(consts: impl Iterator<Item = (&'a Constant, &'a i32)>) -> String {
            consts.map(|(constant, count)| format!(
                "{}{}",
                constant_str(constant),
                if count.abs() <= 1 {String::new()} else {format!("^{}", count.abs())} // Absolute because at this point constants are already sorted inte numerators and denominators.
            )).collect::<Vec<_>>().join("")
        }
        // TODO: Is double-dereferencing ** bad? Figure out if it should be avoided. I'm tired.
        let numerator_consts = join_consts_str(self.consts.iter().filter(|(_constant, count)| **count > 0)); 
        let denominator_consts = join_consts_str(self.consts.iter().filter(|(_constant, count)| **count < 0)); 
        format!("{}{}{}{}{}{}",
            match self.positive {true => "", false => "-"},
            if numerator_consts.is_empty() || self.numerator != 1 {self.numerator.to_string()} else {String::new()},
            numerator_consts,
            if self.denominator != 1 || !denominator_consts.is_empty() {"/"} else {""},
            match self.denominator {1 => String::new(), denominator => denominator.to_string()},
            denominator_consts
        )
    }

    pub fn to_float(&self) -> f64 {
        let mut result = self.numerator as f64;
        if self.denominator != 0 {
            result /= self.denominator as f64;
        }
        for (constant, count) in self.consts.iter() {
            result *= f64::powi(constant_float(constant), *count); 
        }
        if !self.positive {
            result *= -1.;
        }
        // TODO: Implement powers.
        result
    }
}
impl Default for Fraction {
    fn default() -> Fraction {
        Fraction {
            numerator: 0,
            denominator: 1,
            consts: HashMap::new(),
            positive: true,
            power: None
        }
    }
}
// TODO: Always simplify to either pi or tao, store user preference or something.
/// Combines two constant maps, either adding or subtracting the counts a_consts +/- b_consts.
/// The merged map is returned along with a scaling factor s, to be multiplied with the result as
/// R * 2^s.
///
/// Examples:
///     e^4 / e^2 = e^2 * 2^0, resulting in ({e: 2}, 0)
///     tau / pi^2 = pi^-1 * 2^1, resulting in ({pi: -1}, 1)
///     tau^2 / tau^4 = pi^-2 * 2^-2, resulting in ({pi: -2}, -2)
fn merge_constants(
    a_consts: &HashMap<Constant, i32>, 
    b_consts: &HashMap<Constant, i32>,
    use_tau: bool
) -> (HashMap<Constant, i32>, i32) {
    let mut consts = a_consts.clone(); // TODO: Improve efficiency (skip cloning when not needed).
    let mut add_constant = |constant: &Constant, count: i32| {
        match consts.get(constant) {
            Some(a_count) => {
                if *a_count != count {consts.insert(*constant, *a_count + count);}
                else {consts.remove(constant);}
            },
            None => {consts.insert(*constant, count);}
        }
    };
    for (constant, b_count) in b_consts.iter() {
        add_constant(constant, *b_count);
    }
    /*match use_tau {
        false => {
            let count = consts.remove(&Constant::Tau).unwrap_or(0);
            if count != 0 {add_constant(&Constant::Pi, count);}
            (consts, count)
        },
        true => {
            let count = consts.remove(&Constant::Pi).unwrap_or(0);
            if count != 0 {add_constant(&Constant::Tau, count);}
            (consts, -count)
        }
    }*/(consts, 0) // TODO: Re-implement and fix memory ownership issues.
}

pub fn fractinate(expression: &Exp) -> Fraction {
    match expression {
        Exp::Term(a, b) => {
            let (a_frac, b_frac) = (fractinate(a), fractinate(b));
            let gcd = greatest_common_divisor(a_frac.denominator, b_frac.denominator);
            let a_num = a_frac.numerator * b_frac.denominator / gcd;
            let b_num = b_frac.numerator * a_frac.denominator / gcd;
            // TODO: Implement constants.
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
            let mut numerator = a_frac.numerator * b_frac.numerator;
            let mut denominator = a_frac.denominator * b_frac.denominator;
            
            let (consts, scaling_factor) = merge_constants(&a_frac.consts, &b_frac.consts, false);
            if scaling_factor > 0 {numerator *= 2 << scaling_factor;}
            if scaling_factor < 0 {denominator *= 2 << scaling_factor.abs();}
            
            // TODO: Check if this can be made more efficient. (gcd before multiplication?)
            let gcd = greatest_common_divisor(numerator, denominator); 
            numerator /= gcd;
            denominator /= gcd;
            
            Fraction {
                numerator: numerator,
                denominator: denominator,
                consts: consts,
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
                ..a_frac
            }
        },
        Exp::Negative(a) => {
            let frac = fractinate(a);
            Fraction {
                positive: !frac.positive,
                ..frac
            }
        },
        Exp::Inverse(a) => {
            let frac = fractinate(a);
            if frac.numerator == 0 {panic!("Division by zero.");} // TODO: Better error handling.
            Fraction {
                numerator: frac.denominator,
                denominator: frac.numerator,
                consts: frac.consts.iter().map(|(constant, count)| (*constant, -count)).collect(),
                ..frac
            }
        },
        Exp::Number(value) => {
            Fraction {
                numerator: value.digits,
                denominator: 10_u32.pow(value.decimals),
                ..Default::default()
            }
        },
        Exp::Constant(constant) => {
            Fraction {
                numerator: 1,
                consts: HashMap::from([(*constant, 1)]),
                ..Default::default()
            }
        }
    }
}
