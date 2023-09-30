use std::fs;

fn main() {
    let contents = read_line("input.txt");
    println!("Hello, world!");
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
