use std::str::Chars;
use crate::symbols::{exp::Exp/*, constants::Constant*/, fpnum::FixedPointNumber};
use crate::parsing::util::{splitting, cleaning::remove_whitespace};

fn parse_number(equation: &str) -> Exp {
    Exp::Number(equation.parse().unwrap())
}

fn parse_constant(name: char) -> Exp {
    Exp::Number(FixedPointNumber::default())
}

fn parse_constant_chars(mut chars: Chars<'_>) -> Option<Exp> {
    match chars.next() {
        Some(char_current) => match parse_constant_chars(chars) {
            Some(exp_next) => Some(Exp::Factor(Box::from(parse_constant(char_current)), Box::from(exp_next))),
            None => Some(parse_constant(char_current))
        },
        None => None
    }
}
fn parse_constants(equation: &str) -> Exp {
    parse_number(equation)
    /*match parse_constant_chars(equation.chars()) {
        Some(exp) => exp,
        None => panic!("Attempted to parse empty character sequence \"{}\"", equation)
    }*/
}

fn parse_nested(equation: &str) -> Exp {
    let mut actual_equation = equation;
    let mut negate = false;
    if equation.chars().next().unwrap() == '-' {
        negate = true;
        actual_equation = &equation[1..];
    }
    match equation.find('(') {
        Some(_) => {
            let mut chars = actual_equation.chars();
            chars.next();
            chars.next_back();
            match negate {
                true => Exp::Negative(Box::from(parse_term(chars.as_str(), false))),
                false => parse_term(chars.as_str(), false)
            }
        },
        None => {
            match negate {
                true => Exp::Negative(Box::from(parse_constants(actual_equation))),
                false => parse_constants(actual_equation)
            }
        }
    }
}

fn parse_power(equation: &str) -> Exp {
    match splitting::split_at(equation, '^', '√') {
        splitting::Split::Normal((a, b)) => Exp::Pow(Box::from(parse_nested(a)), Box::from(parse_power(b))),
        splitting::Split::Opposite((a, b)) => Exp::Pow(Box::from(parse_nested(b)), Box::from(Exp::Inverse(Box::from(parse_power(a))))),
        splitting::Split::Single(a) => parse_nested(a)
    }
}

fn parse_factor(equation: &str) -> Exp {
    match splitting::split_at(equation, '*', '/') {
        splitting::Split::Normal((a, b)) => Exp::Factor(Box::from(parse_power(a)), Box::from(parse_factor(b))),
        splitting::Split::Opposite((a, b)) => Exp::Factor(Box::from(parse_power(a)), Box::from(Exp::Inverse(Box::from(parse_factor(b))))),
        splitting::Split::Single(a) => parse_power(a)
    }
}

fn parse_term(equation: &str, negate: bool) -> Exp {
    let negated = |exponent: Exp| { // TODO: Do this in a less disgusting way.
        if negate {
            return Exp::Negative(Box::from(exponent))
        }
        exponent
    };
    match splitting::split_at(equation, '+', '-') {
        splitting::Split::Normal((a, b)) => Exp::Term(Box::from(negated(parse_factor(a))), Box::from(parse_term(b, false))),
        splitting::Split::Opposite((a, b)) => Exp::Term(Box::from(negated(parse_factor(a))), Box::from(parse_term(b, true))),
        splitting::Split::Single(a) => negated(parse_factor(a))
    }
}

pub fn parse(equation: &str) -> Exp {
    parse_term(&remove_whitespace(equation), false)
}

