use std::io;
use std::env;

mod util;
mod parsing;
mod calculation;

pub use crate::calculation::calculatinator;
pub use crate::calculation::printinator;
pub use crate::calculation::fractinator;

fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	println!("Calculatinator™");
	loop {
		print!("> ");
		let mut output = String::new();
		io::Write::flush(&mut io::stdout()).unwrap();
		io::stdin().read_line(&mut output).unwrap();

		let equation = parsing::parser::parse(&output);

		println!("{}", printinator::printiate(&equation, false));

		println!("= {}", calculatinator::calculatinate(&equation));

		let frac = fractinator::fractinate(&equation);
		let sign = match frac.positive {true => "", false => "-"};
		let denom = match frac.denominator {1 => String::new(), denominator => format!("/{}", denominator)};
		println!("= {}{}{} ({}{})", sign, frac.numerator, denom, sign, frac.numerator as f64 / frac.denominator as f64);

		if output == "" {
			break;
		}
	}
}
