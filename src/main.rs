use std::io;
use std::env;

mod util;
mod parsing;
mod calculation;

pub use crate::calculation::calculatinator;
pub use crate::calculation::printinator;
pub use crate::calculation::fractinator;

fn print_answer(input: &str) {
	let equation = parsing::parser::parse(&input);
	let frac = fractinator::fractinate(&equation);
	let printed = printinator::printiate(&equation, false);

	let parenthesis = match frac.denominator {1 => String::new(), _ => format!(" ({})", calculatinator::calculatinate(&equation))};
		
	println!("{} = {}{}", printed, frac.to_str(), parenthesis);

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
