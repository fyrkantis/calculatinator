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

/*pub mod cleaning {
	pub fn remove_whitespace(string: &str) -> String {
		string.chars().filter(|character| !character.is_whitespace()).collect() // https://stackoverflow.com/a/57063944
	}
}*/
