// env for access to environment variables, fs to access the file system
use std::env;
use std::io;
use std::path::PathBuf;
use clap::Parser;

mod fs_functions;
mod search;
mod arg_parser;
mod file_opener;

fn main() {
	let args = arg_parser::Args::parse();
	// is this the right place to dereference the string???
	let kw: &str = &args.search;
	let current_path: PathBuf = if args.path != ".".to_string() {
		PathBuf::from(&args.path)
	} else {
		match env::current_dir() {
			Ok(path) => path,
			Err(error) => panic!("Error accessing the current directory: {error:?}"),
		}
	};
	
	let crawler = fs_functions::DirCrawler {
		recurse: args.recurse,
	};
	
	let dir_entries = match if crawler.recurse {
		crawler.recursive_visit_dir(current_path.as_path())
	} else {
		crawler.visit_dir(current_path.as_path())
	} {
		Ok(entries) => entries,
		Err(error) => panic!("Error listing directory: {error:?}"),
	};	
	
	let entries_found = search::search_entries(kw, dir_entries);
	println!("Enter file number to open or quit to exit the program");
	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let user_input = user_input.trim();
	
	if user_input != "quit" {
		let file_number: usize = match user_input.parse() {
			Ok(n) => n,
			Err(error) => panic!("Error reading input from user: {}", error),
		};
		file_opener::open_file(&entries_found[file_number]);
	}
}