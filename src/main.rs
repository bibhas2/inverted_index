mod index;

use index::{InvertedIndex};
use std::env;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    
    if args.len() < 2 {
        println!("Usage: inverted_index keyword files...");
        
        return;
    }
    
    let keyword = &args[0];
    let mut idx = InvertedIndex::new();
    
    for file in (&args).into_iter().skip(1) {
        idx.index_file(file);
    }
    idx.search(keyword);
}
