pub fn encode(source: &str) -> String {
	let len = source.len();

	let mut chars = source.chars();
	let mut result = String::new();
	let mut cnt = 1;

	let mut next = chars.next();
	let mut curr;
	for _ in 0..len{
		curr = next;
		next = chars.next();
		if curr == next {
			cnt += 1;
		} else {
			if cnt != 1{
				result.push_str(&cnt.to_string());
			}
			result.push(curr.unwrap());
			cnt = 1;
		}
	}
	result
}

pub fn decode(source: &str) -> String {

	let chars = source.chars();
	let mut cnt: usize = 0;
	let mut result = String::new();

	for c in chars{
		match c {
			'0'...'9' => cnt = cnt * 10 + (c as usize - 48),
			c => {
				if cnt == 0 { cnt = 1; }
				result.push_str(
					&std::iter::repeat(c).take(cnt).collect::<String>()
				);
				cnt = 0;
			},
		};
	}
	result
}
