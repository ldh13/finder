use open;

pub fn open_file(path: &str) -> () {
	match open::that(path) {
		Ok(()) => println!("File '{}' opened successfully", path),
		Err(error) => println!("An error occurred when opening file: '{}': {}",
						path, error),
	};
}
