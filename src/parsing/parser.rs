use crate::symbols::{exp::Exp, constants::Constant, fpnum::FixedPointNumber};
use crate::parsing::util::{splitting, cleaning::remove_whitespace};

fn parse_number(equation: &str) -> Exp {
    Exp::Number(equation.parse().unwrap())
}

fn parse_constants(equation: &str) -> Exp {
    const CONSTS: [(&str, Constant); 2] = [
        ("pi", Constant::Pi),
        ("π", Constant::Pi)
    ]; 
    let eq = equation.to_lowercase();
    for (name, constant) in CONSTS.iter() {
        if eq.starts_with(name) {
           if eq.chars().count() <= name.chars().count() { // No more characters after this.
                return Exp::Constant(*constant)
           } else { // There are more characters after this.
                return Exp::Factor(
                    Box::from(Exp::Constant(*constant)),
                    Box::from(parse_constants(&eq[name.chars().count()..]))
                )
            }
        }
    }
    parse_number(equation)
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

