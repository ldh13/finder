use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "finder", version = "1.0",
	about = "Cli tool to search and open files", long_about = None)]
pub struct Args {
	// short -p and long --path flags allowed
	#[arg(short, long, help = "Path of the directory to search",
		default_value_t = String::from("."))]
	pub path: String,
	#[arg(short, long, help = "String to match")]
	pub search: String,
	// if the flag is not passed it defaults, if passed it changes to true
	#[arg(short, long,
		help = "Searches subdirectories recursively", default_value_t = false)]
	pub recurse: bool,
}
