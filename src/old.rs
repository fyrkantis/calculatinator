use std::io;
use std::collections::LinkedList;

// Allows for free conversion between f64 and factors. :D
// https://stackoverflow.com/a/70055407/13347795
pub trait FactorLike {
	fn to_f64(self) -> f64;
}
impl FactorLike for f64 {
	fn to_f64(self) -> f64 {self}
}

struct Factor {

}


struct Term {
	// Allows the list to contain anything with the FactorLike trait.
	// https://stackoverflow.com/a/25819164/13347795
	factors: LinkedList<Box<dyn FactorLike>>,
}
impl Term {		
	fn new<T: FactorLike>(value: T) -> Term {
		Term {
			factors: LinkedList::from([Box::new(value)])
		}
	}
	fn add(&self, value: f64) {
			
	}// DO THIS: https://stackoverflow.com/a/70055407/13347795
	fn sum(&self) -> f64 {
		let sum = 0.;
		for factor in self.factors {
			sum += factor.to_f64();
		}
		return sum;
	}
}

fn main() {
	println!("Dave's Fantastic Calculatinator\u{2122} (made in Rust)");
	//let term1 = Term::new(input_number_prompt("Term 1"));
	//let term2 = Term::new(input_number_prompt("Term 2"));

	//println!("{} + {} = {}", term1.value, term2.value, term1.value + term2.value);
	input_prompt("Press enter to exit");
}	

fn input_prompt(prompt: &str) -> String {
	print!("{}> ", prompt);
	io::Write::flush(&mut io::stdout()).unwrap(); // Makes sure "> " is actually written before asking for input.

	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	return line.trim().to_string();
}
fn input() -> String {
	return input_prompt("");
}
fn input_number_prompt(prompt: &str) -> f64 {
	return input_prompt(prompt).parse::<f64>().unwrap();
}
fn input_number() -> f64 {
	return input_number_prompt("Number");
}