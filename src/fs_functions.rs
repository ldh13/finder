use std::fs;
use std::path::{Path,};
use core::result::Result;
use std::io::Error;

pub struct DirCrawler {
	pub recurse: bool,
}

impl DirCrawler {
	pub fn visit_dir(&self, dir: &Path) -> Result<Vec<String>, Error> {
		let mut paths: Vec<String> = Vec::new();
		if dir.is_dir() {
			for entry in fs::read_dir(dir)? {
				let entry = entry?;
				// Need to ensure this is the right way to convert to string
				paths.push(entry.path().to_string_lossy().to_string());
			}
		}
		Ok(paths)
	}
	
	pub fn recursive_visit_dir(&self, dir: &Path) -> Result<Vec<String>, Error> {
		let mut paths: Vec<String> = Vec::new();
		if dir.is_dir() {
			for entry in fs::read_dir(dir)? {
				let entry = entry?;
				if entry.path().is_dir() {
					match self.recursive_visit_dir(&entry.path()) {
						Ok(mut value) => paths.append(&mut value),
						Err(error) => println!("Error encountered {}", error),
					}
				} else {
					paths.push(entry.path().to_string_lossy().to_string());
				}
			}
		}
		Ok(paths)
	}
}