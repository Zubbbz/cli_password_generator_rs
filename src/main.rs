use std::{char, env};

use rand::Rng;

fn main() {
	let charset_alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
	let charset_alphabet_upper: &str = &charset_alphabet_lower.to_uppercase();
	let charset_numbers: &str = "1234567890";

	// ARGS
	let args: Vec<String> = env::args().collect();

	let mut i = 0;
	for arg in &args {
		if arg == "-l" && (i + 1 <= args.len()) {
			match args[i + 1].parse::<u32>() {
				Ok(len) => {
					println!("length: {}", len);
				}
				Err(_) => panic!("Please enter a valid length (u32)"),
			};
		}

		i += 1;
	}

	let gen_charset: String = format!(
		"{}{}{}",
		charset_alphabet_lower, charset_alphabet_upper, charset_numbers
	);
}

fn random_character(charset: String) -> char {
	let mut rng = rand::thread_rng();
	let charset_length = charset.len();
	let random_index: usize =
		(rng.gen::<f64>() * charset_length as f64).floor() as usize;
	return charset.chars().nth(random_index).unwrap();
}
