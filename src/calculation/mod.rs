pub mod calculatinator {
	pub use crate::util::exp::Exp;

	pub fn calculatinate(expression: &Exp) -> f64 {
		match expression {
			Exp::Term(a, b) => calculatinate(a) + calculatinate(b),
			Exp::Factor(a, b) => calculatinate(a) * calculatinate(b),
			Exp::Pow(a, b) => f64::powf(calculatinate(a), calculatinate(b)),
			Exp::Negative(a) => -calculatinate(a),
			Exp::Inverse(a) => f64::powi(calculatinate(a), -1),
			Exp::Number(value) => *value
		}
	}
}

pub mod printinator {
	pub use crate::util::exp::Exp;

	pub fn printiate(expression: &Exp, parenthesize: bool) -> String {
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
}

pub mod fractinator {
	pub use crate::util::exp::Exp;

	pub struct Fraction {
		pub numerator: u32,
		pub denominator: u32,
		pub positive: bool
	}
	impl Fraction {
		pub fn to_str(&self) -> String {
			let sign = match self.positive {true => "", false => "-"};
			let denom = match self.denominator {1 => String::new(), denominator => format!("/{}", denominator)};
			format!("{}{}{}", sign, self.numerator, denom)
		}

		pub fn to_float(&self) -> f64 {
			self.numerator as f64 / self.denominator as f64
		}
	}

	pub fn fractinate(expression: &Exp) -> Fraction {
		match expression {
			Exp::Term(a, b) => {
				let (a_frac, b_frac) = (fractinate(a), fractinate(b));
				let (a_num, b_num) = (a_frac.numerator * b_frac.denominator, b_frac.numerator * a_frac.denominator); // TODO: Make more efficient by looking for common denominators.
				let adding = a_frac.positive == b_frac.positive;
				Fraction {
					numerator: match adding {
						true => a_num + b_num,
						false => a_num.abs_diff(b_num)
					},
					denominator: a_frac.denominator * b_frac.denominator,
					positive: match adding { // TODO: Replace this mess of nested match statements.
						true => a_frac.positive,
						false => match a_num > b_num {
							true => a_frac.positive,
							false => b_frac.positive
						}
					}
				}
			},
			Exp::Factor(a, b) => {
				let (a_frac, b_frac) = (fractinate(a), fractinate(b));
				Fraction {
					numerator: a_frac.numerator * b_frac.numerator,
					denominator: a_frac.denominator * b_frac.denominator,
					positive: a_frac.positive == b_frac.positive
				}
			},
			Exp::Pow(a, b) => todo!(), // TODO: Implement!
			Exp::Negative(a) => {
				let frac = fractinate(a);
				Fraction {
					numerator: frac.numerator,
					denominator: frac.denominator,
					positive: !frac.positive
				}
			},
			Exp::Inverse(a) => {
				let frac = fractinate(a);
				Fraction {
					numerator: frac.denominator,
					denominator: frac.numerator,
					positive: frac.positive
				}
			},
			Exp::Number(value) => {
				Fraction {
					numerator: value.abs() as u32,
					denominator: 1, // TODO: Handle decimal numbers by adding factors of 10 to the denominator.
					positive: *value >= 0.
				}
			}
		}
	}
}
