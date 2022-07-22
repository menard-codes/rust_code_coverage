pub fn calc(x: i32, y: i32, op: &str) -> i32 {
	match op {
		"+" => x + y,
		"-" => x - y,
		"*" => x * y,
		"/" => x / y,
		_ => panic!("Unknown operator")
	}
}

#[cfg(test)]
mod tests {
	use super::calc;

	#[test]
	fn it_adds() {
		assert_eq!(calc(4, 5, "+"), 9);
	}

	#[test]
	fn it_subtracts() {
		assert_eq!(calc(8, 3, "-"), 5);
	}

	#[test]
	fn it_multiplies() {
		assert_eq!(calc(3, 7, "*"), 21);
	}

	#[test]
	fn it_divides() {
		assert_eq!(calc(20, 5, "/"), 4);
	}

	#[test]
	#[should_panic]
	fn it_should_panic() {
		calc(6, 7, "%");
	}
}
