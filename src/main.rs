mod node;
mod parser;
mod token;
mod tokenizer;

use std::fs;
use tokenizer::Tokenizer;

fn main() {
    let file = fs::read_to_string("./test.html").unwrap();
    let mut t = Tokenizer::new(file);
    t.run();
}
