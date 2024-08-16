use std::io;
use std::env;

enum Exp {
	Term(Box<Exp>, Box<Exp>),
	Factor(Box<Exp>, Box<Exp>),
	Pow(Box<Exp>, Box<Exp>),
	Negative(Box<Exp>),
	Inverse(Box<Exp>),
	Number(f64)
}

enum Split<T> {
	Normal((T, T)),
	Opposite((T, T)),
	Single(T)
}
fn split_at<'a>(string: &'a str, split_character: char, inverse_character: char) -> Split<&'a str> {
	let mut last_char = '™'; // TODO: Eeeewwww!
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
				return Split::Opposite((&string[..i], &string[i+1..])); // TODO: Fix this to handle funny unicode characters with multiple bytes.
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
		Exp::Pow(a, b) => f64::powf(calculatinate(a), calculatinate(b)),
		Exp::Negative(a) => -calculatinate(a),
		Exp::Inverse(a) => f64::powi(calculatinate(a), -1),
		Exp::Number(value) => *value
	}
}

fn printiate(expression: &Exp, parenthesize: bool) -> String {
	let parenthesis = |input: String| -> String {
		if parenthesize {
			return format!("({})", input)
		}
		input
	};
	match expression {
		Exp::Term(a, b) => {
			match b.as_ref() {
				Exp::Negative(b_child) => parenthesis(format!("{}-{}", printiate(&a, false), printiate(&b_child, true))),
				Exp::Term(b_child_a, _b_child_b) => {
					match b_child_a.as_ref() {
						Exp::Negative(_) => parenthesis(format!("{}{}", printiate(&a, false), printiate(&b, false))),
						_ => parenthesis(format!("{}+{}", printiate(&a, false), printiate(&b, false)))
					}
				}
				_ => parenthesis(format!("{}+{}", printiate(&a, false), printiate(&b, false)))
			}
		},
		Exp::Factor(a, b) => {
			match b.as_ref() {
				Exp::Inverse(b_child) => format!("{}/{}", printiate(&a, true), printiate(&b_child, true)),
				_ => format!("{}*{}", printiate(&a, true), printiate(&b, true))
			}
		},
		Exp::Pow(a, b) => parenthesis(format!("{}^{}", printiate(&a, true), printiate(&b, true))),
		Exp::Negative(a) => parenthesis(format!("-{}", printiate(&a, true))),
		Exp::Inverse(a) => format!("1/{}", printiate(&a, true)),
		Exp::Number(value) => format!("{}", value)
	}
}

/*fn fractinate(expression: &Exp) -> Exp {
	fn sum(a: Exp, b: Exp) -> Exp {

	}
	match expression {
		Exp::Term(a, b) => sum(fractinate(&a), fractinate(&b)),
		Exp::Factor(a, b) =>
	}
}*/

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
	match split_at(equation, '^', '√') {
		Split::Normal((a, b)) => Exp::Pow(Box::from(parse_nested(a)), Box::from(parse_power(b))),
		Split::Opposite((a, b)) => Exp::Pow(Box::from(parse_nested(b)), Box::from(Exp::Inverse(Box::from(parse_power(a))))),
		Split::Single(a) => parse_nested(a)
	}
}

fn parse_factor(equation: &str) -> Exp {
	match split_at(equation, '*', '/') {
		Split::Normal((a, b)) => Exp::Factor(Box::from(parse_power(a)), Box::from(parse_factor(b))),
		Split::Opposite((a, b)) => Exp::Factor(Box::from(parse_power(a)), Box::from(Exp::Inverse(Box::from(parse_factor(b))))),
		Split::Single(a) => parse_power(a)
	}
}

fn parse_term(equation: &str, negate: bool) -> Exp {
	let negated = |exponent: Exp| {
		if negate {
			return Exp::Negative(Box::from(exponent))
		}
		exponent
	};
	match split_at(equation, '+', '-') {
		Split::Normal((a, b)) => Exp::Term(Box::from(negated(parse_factor(a))), Box::from(parse_term(b, false))),
		Split::Opposite((a, b)) => Exp::Term(Box::from(negated(parse_factor(a))), Box::from(parse_term(b, true))),
		Split::Single(a) => negated(parse_factor(a))
	}
}

fn remove_whitespace(string: &str) -> String {
	string.chars().filter(|character| !character.is_whitespace()).collect() // https://stackoverflow.com/a/57063944
}

fn parse(equation: &str) -> Exp {
	parse_term(&remove_whitespace(equation), false)
}

fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	println!("Calculatinator™");
	loop {
		print!("> ");
		let mut output = String::new();
		io::Write::flush(&mut io::stdout()).unwrap();
		io::stdin().read_line(&mut output).unwrap();
		let equation = parse(&output);
		println!("{}", printiate(&equation, false));
		println!("= {}", calculatinate(&equation));

		if output == "" {
			break;
		}
	}
}
