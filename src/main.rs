use std::io;

enum Exp {
	Term(Box<Exp>, Box<Exp>),
	Factor(Box<Exp>, Box<Exp>),
	Number{value: f64, negate: bool, invert: bool},
	Nested{exp: Box<Exp>, negate: bool, invert: bool}
}

enum Split<T> {
	Normal((T, T)),
	Inverse((T, T)),
	Single(T)
}
fn split_at<'a>(string: &'a str, split_character: char, inverse_character: char) -> Split<&'a str> {
	let mut last_char = '™';
	let mut nesting_depth: usize = 0;
	for (i, character) in string.trim().chars().enumerate() {
		if character == '(' {
			nesting_depth += 1;
		}
		if nesting_depth > 0 {
			if character == ')' {
				nesting_depth -= 1;
			}
			continue;
		}
		if i > 0 && last_char != '*' && last_char != '/' { // Cases where the sign should be grouped with the number itself.
			if character == split_character {
				return Split::Normal((&string[..i], &string[i+1..]));
			} else if character == inverse_character {
				return Split::Inverse((&string[..i], &string[i+1..]));
			}
		}
		last_char = character;
	}
	Split::Single(string)
}

fn calculatinate(expression: &Exp) -> f64 {
	match expression {
		Exp::Term(a, b) => calculatinate(a) + calculatinate(b),
		Exp::Factor(a, b) => calculatinate(a) * calculatinate(b),
		Exp::Number{value, negate, invert} => {
			let mut number = *value;
			if *negate {
				number *= -1.;
			}
			if *invert {
				number = f64::powi(number, -1)
			}
			number
		}
		Exp::Nested{exp, negate, invert} => {
			let mut number = calculatinate(exp);
			if *negate { // TODO: Combine with last somehow, or actually maybe not.
				number *= -1.;
			}
			if *invert {
				number = f64::powi(number, -1);
			}
			number
		}
	}
}

fn parse_number(equation: &str, negate: bool, invert: bool) -> Exp {
	println!("{}", equation);
	Exp::Number{
		value: equation.parse().unwrap(),
		negate: negate,
		invert: invert
	}
}

fn parse_nested(equation: &str, negate: bool, invert: bool) -> Exp {
	let mut actually_negate = negate; // TODO: Do this prooperly.
	let mut actual_equation = equation;
	if equation.chars().next().unwrap() == '-' {
		actually_negate = !negate;
		actual_equation = &equation[1..];
	}
	match equation.find('(') {
		Some(_) => {
			let mut chars = actual_equation.chars();
			chars.next();
			chars.next_back();
			Exp::Nested{
				exp: Box::new(parse(chars.as_str())),
				negate: actually_negate,
				invert: invert
			}
		},
		None => parse_number(actual_equation, actually_negate, invert)
	}
}

fn parse_factor(equation: &str, negate: bool, invert: bool) -> Exp {
	match split_at(equation, '*', '/') {
		Split::Normal((a, b)) => Exp::Factor(Box::from(parse_nested(a, negate, invert)), Box::from(parse_factor(b, false, false))),
		Split::Inverse((a, b)) => Exp::Factor(Box::from(parse_nested(a, negate, invert)), Box::from(parse_factor(b, false, true))),
		Split::Single(a) => parse_nested(a, negate, invert)
	}
}

fn parse_term(equation: &str, negate: bool) -> Exp {
	match split_at(equation, '+', '-') {
		Split::Normal((a, b)) => Exp::Term(Box::from(parse_factor(a, negate, false)), Box::from(parse_term(b, false))),
		Split::Inverse((a, b)) => Exp::Term(Box::from(parse_factor(a, negate, false)), Box::from(parse_term(b, true))),
		Split::Single(a) => parse_factor(a, negate, false)
	}
}

fn remove_whitespace(string: &str) -> String {
	string.chars().filter(|character| !character.is_whitespace()).collect() // https://stackoverflow.com/a/57063944
}

fn parse(equation: &str) -> Exp {
	parse_term(&remove_whitespace(equation), false)
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
