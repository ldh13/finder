pub fn search_entries(kw: &str, entries: Vec<String>) -> Vec<String> {
	let filtered_entries: Vec<String> = entries
		.into_iter()
		.filter(|entry| entry.to_lowercase().contains(kw))
		.collect();
		
	for (entry, index) in filtered_entries.clone().into_iter().enumerate() {
		println!("[{}] {}", entry, index);
	}
	filtered_entries
}