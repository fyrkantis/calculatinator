#[cfg(test)]
mod tests {
	use crate::symbols::exp;
	use crate::parsing::parser::*;

	#[test]
	fn test_terms() {
		assert_eq!(parse("1+1"), 4)
	}
}
