pub mod fpnum {
	
	/// A positive number number with set number of decimals: digits / 10^decimals
	pub struct FixedPointNumber {
		pub digits: u32,
		pub decimals: u32
	}

	#[derive(Debug, PartialEq, Eq)]
	pub struct ParseFixedPointNumberError {
		pub e: std::num::ParseIntError
	}

	impl FixedPointNumber {
		fn parse_split((a, b): (&str, &str)) -> Result<Self, ParseFixedPointNumberError> {
			let mut digits_string = a.to_owned();
			digits_string.push_str(b);
			match digits_string.parse::<u32>() {
				Ok(digits) => Ok(FixedPointNumber {digits: digits, decimals: b.chars().count() as u32}),
				Err(e) => Err(ParseFixedPointNumberError {e: e})
			}
			
		}

		pub fn to_float(&self) -> f64 {
			self.digits as f64 / (u32::pow(10, self.decimals) as f64)
		}

		pub fn to_string(&self) -> String {
			self.to_float().to_string()
		}
	}
	impl Default for FixedPointNumber {
		fn default() -> FixedPointNumber {
			FixedPointNumber {
				digits: 0,
				decimals: 0
			}
		}
	}
	impl std::str::FromStr for FixedPointNumber {
		type Err = ParseFixedPointNumberError;
		
		fn from_str(input: &str) -> Result<Self, Self::Err> {
			match input.rsplit_once('.') {
				Some(split) => FixedPointNumber::parse_split(split),
				None => {
					match input.rsplit_once(',') {
						Some(split) => FixedPointNumber::parse_split(split),
						None => match input.parse::<u32>() {
							Ok(digits) => Ok(FixedPointNumber {digits: digits, ..Default::default()}),
							Err(e) => Err(Self::Err {e: e})
						}
					}
				}
			}
		}
	}
}

pub mod exp {
	pub enum Exp {
		/// a + b
		Term(Box<Exp>, Box<Exp>),
		/// a * b
		Factor(Box<Exp>, Box<Exp>),
		/// a^b
		Pow(Box<Exp>, Box<Exp>),
		/// -a
		Negative(Box<Exp>),
		/// 1/a
		Inverse(Box<Exp>),
		Number(crate::util::fpnum::FixedPointNumber)
	}
}

pub mod splitting {
	pub enum Split<T> {
		Normal((T, T)),
		Opposite((T, T)),
		Single(T)
	}
	pub fn split_at<'a>(string: &'a str, split_character: char, inverse_character: char) -> Split<&'a str> {
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
}

pub mod cleaning {
	pub fn remove_whitespace(string: &str) -> String {
		string.chars().filter(|character| !character.is_whitespace()).collect() // https://stackoverflow.com/a/57063944
	}
}
