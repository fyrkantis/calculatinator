use std::io;

enum Exponent {
	Term(Box<Exponent>, Box<Exponent>),
	Factor(Box<Exponent>, Box<Exponent>),
	Number{value: f64, negate: bool, invert: bool}
}

fn find_end_parenthesis(string: &str) -> usize {
	/// Takes a string starting from the character after the parenthesis to be closed.
	let mut depth: u16 = 0;
	for (i, character) in string.chars().enumerate() {
		match character {
			'(' => depth += 1,
			')' => {
				if depth == 0 {
					return i
				}
				depth -= 1;
			}
			_ => {}
		}
	}
	// TODO: Handle error.
	0
}

enum Split<T> {
	Normal((T, T)),
	Inverse((T, T)),
	None
}
fn split_at<'a>(string: &'a str, split_character: char, inverse_character: char) -> Split<&'a str> {
	let mut skip: u16 = 0;
	for (i, character) in string.chars().enumerate() {
		//if character == '(' {
		//	return 
		//}
		if character == split_character {
			if i > 0 {
				return Split::Normal((&string[..i], &string[i+1..]));
			} else {
				continue
			}
		} else if character == inverse_character {
			if i > 0 {
				return Split::Inverse((&string[..i], &string[i+1..]));
			} else {
				
				continue
			}
		}
	}
	Split::None
}

fn calculatinate(expression: &Exponent) -> f64 {
	match expression {
		Exponent::Term(a, b) => calculatinate(a) + calculatinate(b),
		Exponent::Factor(a, b) => calculatinate(a) * calculatinate(b),
		Exponent::Number{value, negate, invert} => {
			let mut number = *value;
			if *negate {
				number *= -1.;
			}
			if *invert {
				number = f64::powi(number, -1)
			}
			number
		}
	}
}

fn parse_number(equation: &str, negate: bool, invert: bool) -> Exponent {
	println!("{}", equation);
	Exponent::Number{
		value: equation.trim().parse().unwrap(),
		negate: negate,
		invert: invert
	}
}


fn parse_factor(equation: &str, negate: bool, invert: bool) -> Exponent {
	match split_at(equation, '*', '/') {
		Split::Normal((a, b)) => Exponent::Factor(Box::from(parse_number(a, negate, invert)), Box::from(parse_factor(b, false, false))),
		Split::Inverse((a, b)) => Exponent::Factor(Box::from(parse_number(a, negate, invert)), Box::from(parse_factor(b, false, true))),
		Split::One(a) => parse_number(a, false, false),
		Split::OneInverse(a) => parse_number(a, false, true),
		Split::None => parse_number(equation, negate, invert)
	}
}

fn parse_term(equation: &str, negate: bool) -> Exponent {
	match split_at(equation, '+', '-') {
		Split::Normal((a, b)) => Exponent::Term(Box::from(parse_factor(a, negate, false)), Box::from(parse_term(b, false))),
		Split::Inverse((a, b)) => Exponent::Term(Box::from(parse_factor(a, negate, false)), Box::from(parse_term(b, true))),
		Split::One(a) => parse_factor(a, negate, false),
		Split::OneInverse(a) => parse_factor(a, !negate, false),
		Split::None => parse_factor(equation, negate, false)
	}
}

fn parse(equation: &str) -> Exponent {
	parse_term(equation, false)
}

fn main() {
	println!("Calculatinator™");
	loop {
		print!("> ");
		let mut output = String::new();
		io::Write::flush(&mut io::stdout()).unwrap();
		io::stdin().read_line(&mut output).unwrap();
		let equation = parse(&output);
		println!("= {}", calculatinate(&equation));

		if output == "" {
			break;
		}
	}
}
