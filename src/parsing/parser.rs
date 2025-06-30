use crate::util::exp::Exp;
use crate::util::splitting;
use crate::util::cleaning::remove_whitespace;

fn parse_number(equation: &str) -> Exp {
    Exp::Number(equation.parse().unwrap())
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
                true => Exp::Negative(Box::from(parse_number(actual_equation))),
                false => parse_number(actual_equation)
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

