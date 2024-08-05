use std::io;

enum Exp {
	Term(Box<Exp>, Box<Exp>),
	Factor(Box<Exp>, Box<Exp>),
	Number(f64),
	Pow(Box<Exp>, Box<Exp>)
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
		Exp::Number(value) => *value,
		Exp::Pow(a, b) => f64::powf(calculatinate(a), calculatinate(b))
	}
}

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
			if negate {
				Exp::Factor(Box::from(parse_term(chars.as_str())), Box::from(Exp::Number(-1.)))
			} else {
				parse_term(chars.as_str())
			}
		},
		None => {
			if negate {
				Exp::Factor(Box::from(parse_number(actual_equation)), Box::from(Exp::Number(-1.)))
			} else {
				parse_number(actual_equation)
			}
		}
	}
}

fn parse_factor(equation: &str) -> Exp {
	match split_at(equation, '*', '/') {
		Split::Normal((a, b)) => Exp::Factor(Box::from(parse_nested(a)), Box::from(parse_factor(b))),
		Split::Inverse((a, b)) => Exp::Factor(Box::from(parse_nested(a)), Box::from(Exp::Pow(Box::from(parse_factor(b)), Box::from(Exp::Number(-1.))))),
		Split::Single(a) => parse_nested(a)
	}
}

fn parse_term(equation: &str) -> Exp {
	match split_at(equation, '+', '-') {
		Split::Normal((a, b)) => Exp::Term(Box::from(parse_factor(a)), Box::from(parse_term(b))),
		Split::Inverse((a, b)) => Exp::Term(Box::from(parse_factor(a)), Box::from(Exp::Factor(Box::from(parse_term(b)), Box::from(Exp::Number(-1.))))),
		Split::Single(a) => parse_factor(a)
	}
}

fn remove_whitespace(string: &str) -> String {
	string.chars().filter(|character| !character.is_whitespace()).collect() // https://stackoverflow.com/a/57063944
}

fn parse(equation: &str) -> Exp {
	parse_term(&remove_whitespace(equation))
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
