pub fn greatest_common_divisor(input1: u32, input2: u32) -> u32 {
	// Sorts inputs so that a is greater than b.
	let (mut a, mut b) = match input1 > input2 {true => (input1, input2), false => (input2, input1)};

	// Division by zero, no common divisor.
	if b <= 0 {return 1} // TODO: Prettier solution. 

	// Euclidean algorithm
	// a = b * k + m
	let mut m_previous = b;
	loop {
		let m = a % b;
		if m == 0 {return m_previous}
		
		a = b;
		b = m;
		m_previous = m;
	}
}
