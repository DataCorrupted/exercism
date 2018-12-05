use std::iter;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
	let x = input.len();
	let y = input[0].len();
	let mut result = Vec::new();

	let mut row_max: Vec<u64> = iter::repeat(0).take(x).collect();
	let mut col_min: Vec<u64> = iter::repeat(std::u64::MAX).take(y).collect();
	for i in 0..x {
		for j in 0..y{
			if input[i][j]>row_max[i]{
				row_max[i] = input[i][j]
			}
			if input[i][j]<col_min[j]{
				col_min[j] = input[i][j]
			}
		}
	}
	for i in 0..x {
		for j in 0..y {
			if col_min[j] == row_max[i] {
				result.push((i, j));
			}
		}
	}

	result
}
