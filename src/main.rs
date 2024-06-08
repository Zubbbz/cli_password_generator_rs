use std::char;

use clap::Parser;

use rand::Rng;

struct Password {
	length: u128,
	charset: String,
}

impl Password {
	pub fn new(length: Option<u128>, charset: String) -> Self {
		match length {
			Some(len) => Self {
				length: len,
				charset,
			},
			None => Self { length: 8, charset },
		}
	}

	pub fn generate(&self) -> String {
		let mut pw: String = String::from("");
		for _ in 0..self.length {
			pw.push(random_character(&self.charset));
		}
		pw
	}
}

struct Charset {
	alphabet_lower: bool,
	alphabet_upper: bool,
	numbers: bool,
	symbols: bool,
}

impl Charset {
	pub fn default() -> Self {
		Self {
			alphabet_lower: true,
			alphabet_upper: true,
			numbers: true,
			symbols: false,
		}
	}

	pub fn get(&self) -> String {
		let mut set: String = "".to_owned();
		if self.alphabet_lower == true {
			set.push_str("abcdefghijklmnopqrstuvwxyz")
		}
		if self.alphabet_upper == true {
			set.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
		}
		if self.numbers == true {
			set.push_str("1234567890");
		}
		if self.symbols == true {
			set.push_str("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
		}
		set
	}
}

/// A simple program to generate passwords
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	// ? Default values are done elsewhere, currently not using defaults for the args, as option type doesnt implement display...
	/// Length of password
	#[arg(short, long)]
	length: Option<u128>,

	/// Enable lowercase letters
	#[arg(long)]
	lower: Option<bool>,

	/// Enable Uppercase letters
	#[arg(long)]
	upper: Option<bool>,

	/// Enable numbers
	#[arg(long)]
	numbers: Option<bool>,

	/// Enable symbols
	#[arg(long)]
	symbols: Option<bool>,
}

fn main() {
	let args = Args::parse();

	let charset = Charset::get(&Charset {
		alphabet_lower: args.lower.unwrap_or(Charset::default().alphabet_lower),
		alphabet_upper: args.upper.unwrap_or(Charset::default().alphabet_upper),
		numbers: args.numbers.unwrap_or(Charset::default().numbers),
		symbols: args.symbols.unwrap_or(Charset::default().symbols),
	});

	let pw = Password::new(args.length, charset).generate();
	println!("Password: {}\nlength: {}", pw, pw.len());
}

fn random_character(charset: &str) -> char {
	let mut rng = rand::thread_rng();
	let charset_length = charset.len();
	let random_index: usize =
		(rng.gen::<f64>() * charset_length as f64).floor() as usize;
	return charset.chars().nth(random_index).unwrap();
}
