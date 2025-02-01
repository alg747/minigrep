use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for:\t\t'{}'", query);
    println!("In the file:\t\t'{file_path}'");

    // dbg!(args);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("With the text:\n---\n{contents}---");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
