use std::io;
use std::env;

use calculatinator::parsing::parser;
use calculatinator::calculation::{
	calculatinator::calculatinate, // TODO: Less horrible naming convention.
	fractinator::fractinate,
	printinator::printinate
};

fn print_answer(input: &str) {
	let equation = parser::parse(&input);
	let frac = fractinate(&equation);
	let printed = printinate(&equation, false);

	let parenthesis = match frac.denominator {/*1 => String::new(), */_ => format!(" ({})", calculatinate(&equation))};

	println!("{} = {}{}", printed, frac.to_string(), parenthesis);
}

fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		print_answer(&args[1..].join(" "));
	} else {
		println!("Calculatinator™");
		loop {
			print!("> ");
			let mut input = String::new();
			io::Write::flush(&mut io::stdout()).unwrap();
			io::stdin().read_line(&mut input).unwrap();

			if input.trim() == "" {
				break;
			}

			print_answer(&input);
		}
	}
}
