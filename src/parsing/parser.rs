use crate::symbols::{exp::Exp, constants::Constant};
use crate::parsing::util::{splitting/*, cleaning::remove_whitespace*/};

fn parse_number(equation: &str) -> Exp {
    Exp::Number(equation.parse().unwrap())
}

/// Recognizes constants and removes whitespace between them.
fn parse_constants(equation: &str) -> Exp {
    /// [(symbol name, name length, Constant type)]
    const CONSTS: [(&str, usize, Constant); 6] = [
        ("pi", 2, Constant::Pi),
        ("π", 1, Constant::Pi),
        ("tau", 3, Constant::Tau),
        ("τ", 1, Constant::Tau),
        ("e", 1, Constant::E),
        ("i", 1, Constant::Imaginary)
    ]; 
    let eq = equation.trim().to_lowercase();
    let mut eq_chars = eq.chars();
    let eq_chars_count = eq_chars.clone().count();
    
    // TODO: Refactor this function. Check for digit/character first.
    // TODO: Use RegEx for constant detection.
    // TODO: Define CONSTS in constants.rs module instead.

    // Check if string starts with a constant, and if so parse it.
    for (name, chars_count, constant) in CONSTS.iter() {
        if eq.starts_with(name) {
            if eq_chars_count <= *chars_count { // No more characters after this.
                return Exp::Constant(*constant)
            } else { // There are more characters after this.
                return Exp::Factor(
                    Box::from(Exp::Constant(*constant)),
                    Box::from(parse_constants(&eq[*chars_count..]))
                )
            }
        }
    }

    // Parses start of string as number. 
    match eq_chars.next() {
        None => panic!("Attempted to parse empty string \"{}\".", equation),
        Some(first_char) => {
            if !first_char.is_ascii_digit() {
                panic!("Attempted to parse unknown symbol at the start of \"{}\".", equation)
            }
            let mut i = 1;
            for digit in eq_chars {
                if !digit.is_ascii_digit() && digit != '.' && digit != ',' {
                    return Exp::Factor(
                        Box::from(parse_number(&eq[..i])), // Parse previous digits as number.
                        Box::from(parse_constants(&eq[i..])) // Continue parsing constants/numbers.
                    )
                }
                i += 1;
            }
            parse_number(&eq) // Parse the entire string as a number.
        }
    }
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
    parse_term(/*&remove_whitespace(*/equation/*)*/, false)
}

