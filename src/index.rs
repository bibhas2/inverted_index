use std::collections::HashMap;
use std::ascii::AsciiExt;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct  IndexItem<'a> {
	pub file_list: Vec<&'a str>,
	pub count: usize
}

impl <'a> IndexItem<'a> {
	pub fn new() ->  IndexItem<'a> {
		IndexItem {
			file_list: Vec::new(),
			count: 0
		}
	}
}

pub struct InvertedIndex<'a> {
	pub index: HashMap<String, IndexItem<'a>>
}

impl <'a> InvertedIndex<'a> {
	pub fn new() -> InvertedIndex<'a> {
		InvertedIndex{
			index: HashMap::new()
		}
	}
	
	pub fn search(&self, word: &str) {
		let word = word.to_ascii_lowercase();
		
		if let Some(item) = self.index.get(&word) {
			println!("Found {} instances of {} in:",
				item.count,
				word);

			for file in &(item.file_list) {
				println!("\t- {}", file);
			}
		}
	}
	
	fn index_word(&mut self, file: &'a str, word: &String) {
		let word = word.to_ascii_lowercase();
		
		if let Some(val) = self.index.get_mut(&word) {
			val.count += 1;
			
			//See if the file is already there
			for f in &(val.file_list) {
				if *f == file {
					return;
				}
			}
			
			val.file_list.push(file);
			
			return;
		} 
		
		let mut item = IndexItem::new();
		
		item.count = 1;
		item.file_list.push(file);
		
		self.index.insert(word.clone(), item);
	}
	
	pub fn index_line(&mut self, file: &'a str, line: &str) {
		let mut start_mode = true;
		let mut word = String::new();
				
		for ch in line.chars() {
			if ch.is_whitespace() {
				if start_mode {
					continue;
				}
				//End of word
				start_mode = true;
				self.index_word(file, &word);
			} else {
				if start_mode {
					start_mode = false;
					word.clear();
				}
				word.push(ch);
			}
		}
		
		if start_mode == false {
			self.index_word(file, &word);
		}
	}
	pub fn index_file(&mut self, file_name: &'a str) {
	    if let Ok(file) = File::open(file_name) {	
		    let mut reader = BufReader::new(file);		
		    let mut line = String::new();
		
		    while let Ok(size) = reader.read_line(&mut line) {
				if size == 0 {
					break;
				}
				
		        self.index_line(file_name, &line);
				line.clear();
		    }	
		} else {
			println!("Failed to open file: {}", file_name);
		}
	}
}