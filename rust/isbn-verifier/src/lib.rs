/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
	let mut pow: i32 = 10;
	let mut sum = 0;
	for c in isbn.chars(){
		match c {
			'0' ... '9' => {
				sum += pow as u32 * c.to_digit(10).unwrap();
				pow -= 1;
			}
			'X' => {
				sum += pow as u32 * 10;
				pow -= 1;				
				if pow != 0 {
					return false;
				}
			}
			_ => {;},
		}
	}
	sum % 11 == 0 && pow == 0
}
