use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query} file {file_path}.");

    let contents =
        fs::read_to_string(file_path).expect("There was a problem with reading the file!");

    println!("With text:\n{contents}");
}
