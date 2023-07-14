mod lexical_analyzer;

use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to monkey interpreter!\n");

    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let tokens = lexical_analyzer::tokenize(&input);
        println!(
            "{:?}",
            tokens.iter().map(|token| format!("{:?}", token)).collect::<Vec<String>>()
        );
    }
}