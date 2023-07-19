use std::{collections::HashSet, fs};

fn main() {
    let contents = read_line("input.txt");
    for line in contents {
        let chars: Vec<String> = line.chars().map(|f| f.to_string()).collect();

        for i in 13..chars.len() {
            let l: HashSet<&String> = HashSet::from_iter(&chars[i - 13..=i]);
            if l.len() == 14 {
                println!("{}", i + 1);
                break;
            }
        }
    }
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
