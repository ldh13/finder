use std::ffi;
use open;

pub fn open_file(path: &str) -> () {
	match open::that(path) {
		Ok(()) => println!("Opened '{}' successfully", path),
		Err(error) => panic!("An error occurred when opening '{}': {}", path, error),
	};
}