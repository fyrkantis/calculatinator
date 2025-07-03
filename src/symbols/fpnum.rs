/// A positive number number with set number of decimals, calculated using: digits / 10^decimals
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct FPNum {
	pub digits: u32,
	pub decimals: u32
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseFPNumError {
	pub e: std::num::ParseIntError
}

impl FPNum {
	fn from_split((a, b): (&str, &str)) -> Result<Self, ParseFPNumError> {
		let mut digits_string = a.to_owned();
		digits_string.push_str(b);
		match digits_string.parse::<u32>() {
			Ok(digits) => Ok(FPNum {digits: digits, decimals: b.chars().count() as u32}),
			Err(e) => Err(ParseFPNumError {e: e})
		}
		
	}

	pub fn to_float(&self) -> f64 {
		self.digits as f64 / (u32::pow(10, self.decimals) as f64)
	}
}
impl ToString for FPNum {
	fn to_string(&self) -> String {
		self.to_float().to_string() // TODO: Make a proper number stringifier and don't involve floats at all.
	}
}
impl Default for FPNum {
	fn default() -> Self {Self {
		digits: 0,
		decimals: 0
	}}
}
impl std::str::FromStr for FPNum {
	type Err = ParseFPNumError;
	
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input.rsplit_once('.') {
			Some(split) => FPNum::from_split(split),
			None => {
				match input.rsplit_once(',') {
					Some(split) => FPNum::from_split(split),
					None => match input.parse::<u32>() {
						Ok(digits) => Ok(FPNum {digits: digits, ..Default::default()}),
						Err(e) => Err(Self::Err {e: e})
					}
				}
			}
		}
	}
}
//impl Ord for FPNum { // TODO: Fraction comparison.
//	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//		self.to_float().cmp(other.to_float())
//	}
//}
//impl PartialOrd for FPNum {fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {Some(self.cmp(other))}}
