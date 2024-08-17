#[cfg(test)]
mod tests {
	use crate::util::exp;
	use crate::parsing::parser::*;

	#[test]
	fn test_terms() {
		assert_eq!(parse("1+1"), 4)
	}
}
