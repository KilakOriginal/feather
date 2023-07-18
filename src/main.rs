use std::fs;
use std::env;

#[derive(Debug)]
struct Lexer {

    input: String,
}

impl Lexer {

    pub fn new(input: String) -> Self {

        Self { input }
    }

    pub fn generate_tokens(&mut self) {}
}

fn main() {

    let file_path = env::args().nth(1);
    let file_path = if file_path.is_some() { file_path.unwrap() } else { panic!("Expected a file.") };

    let input = fs::read_to_string(file_path);
    let input = if input.is_ok() { input.unwrap() } else { panic!("Unable to open file.") };

    let lexer = Lexer::new(input);

    println!("{:?}", lexer);
}

