use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for:\t\t'{}'", config.query);
    println!("In the file:\t\t'{}'", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("With the text:\n---\n{contents}---");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
