use std::{
    cmp::Ordering,
    env,
    io::{self, Write},
};

use crate::lexer::Lexer;

mod lexer;
// use crate::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    // first argument of args is the location of the file, that's why it starts at 2
    match args.len().cmp(&2) {
        Ordering::Greater => panic!("Usage: rlox [script]"),
        // Ordering::Equal => run_file(&args[1]),
        Ordering::Less => run_prompt(),
        _ => (),
    }
}

// pub fn run_file(file_path: &str) {
//     let _file = fs::read_to_string(file_path);
//     todo!()
// }

pub fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        stdin.read_line(&mut buffer).unwrap();

        println!("{:?}", Lexer::scan(&buffer[..(buffer.len() - 1)]));

        buffer = String::new();
    }
}
