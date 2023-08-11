use std::collections::HashSet;
use rand::Rng;

const L: usize = 7;

pub fn genuid(count: usize) -> HashSet<String> {
	let characters: Vec<char> = "0123456789abcdef".chars().collect();
	let mut rng = rand::thread_rng();
	let mut str = HashSet::new();
	let mut result = Vec::new();

	while str.len() < count {
		let random_string: String = (0..L).map(|_| characters[rng.gen_range(0..characters.len())]).collect();

		if str.insert(random_string.clone()) {
			result.push(random_string);
		}
	}
	return str;
}