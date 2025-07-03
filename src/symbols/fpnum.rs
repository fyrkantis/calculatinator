/// A positive number number with set number of decimals: digits / 10^decimals
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct FixedPointNumber {
	pub digits: u32,
	pub decimals: u32
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
