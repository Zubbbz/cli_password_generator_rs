use std::char;

use clap::Parser;

use rand::Rng;

struct Password {
	length: u8,
	charset: String,
}

impl Password {
	pub fn new(length: u8, charset: String) -> Self {
		Self { length, charset }
	}

	pub fn generate(&self) -> String {
		let mut pw: String = String::from("");
		for _ in 0..self.length {
			pw.push(random_character(&self.charset));
		}
		pw
	}
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	// Length of pass
	#[arg(short, long)]
	length: u8,
}

fn main() {
	let charset_alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
	let charset_alphabet_upper: &str = &charset_alphabet_lower.to_uppercase();
	let charset_numbers: &str = "1234567890";

	let args = Args::parse();
	let gen_charset: String = format!(
		"{}{}{}",
		charset_alphabet_lower, charset_alphabet_upper, charset_numbers
	);
	let pw = Password::new(args.length, gen_charset).generate();
	println!("Password: {}\nlength: {}", pw, pw.len());
}

fn random_character(charset: &str) -> char {
	let mut rng = rand::thread_rng();
	let charset_length = charset.len();
	let random_index: usize =
		(rng.gen::<f64>() * charset_length as f64).floor() as usize;
	return charset.chars().nth(random_index).unwrap();
}
