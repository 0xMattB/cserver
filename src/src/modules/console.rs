use std::io::{self, Write, stdout};

pub fn input() -> Result<String, &'static str> {
	let mut read = String::new();
	
	if let Ok(_) = io::stdin().read_line(&mut read) {
		Ok(read)
	} else {
		Err("console::input(): error reading from input")
	}
}

pub fn output(data: &str, newline: bool) {
	print!("{data}");
	
	if newline {
		println!();
	} else {
		let _ = stdout().flush();
	}
}