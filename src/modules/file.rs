use std::fs;

pub fn read(filename: &str) -> Option<String> {
	if let Ok(read) = fs::read_to_string(filename) {
		Some(read)
	} else {
		None
	}
}

pub fn write(filename: &str, data: &str) -> Result<(), &'static str> {
	if let Ok(_) = fs::write(filename, data) {
		Ok(())
	} else {
		Err("file::write(): error writing to file")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn file_read_test() {
		if let Some(r) = read("test.txt") {
			if r == "test file." {
				assert!(true);
			} else {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}
	
	#[test]
	fn file_read_write() {
		if let Ok(()) = write("test.txt", "test file.") {
			assert!(true);
		} else {
			assert!(false);
		}
	}
}
