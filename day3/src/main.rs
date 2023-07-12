use std::fs;

fn main() {
    let mut score = 0;
    let contents = read_line("input.txt");
    let mut i = 0;
    while i < contents.len() {
        let mut c: char = '/';
        for j in contents[i].chars() {
            for k in contents[i + 1].chars() {
                for l in contents[i + 2].chars() {
                    if j == k && j == l {
                        c = j;
                    }
                }
            }
        }
        if 65 <= c as u32 && c as u32 <= 90 {
            score += c as u32 - 38
        } else {
            score += c as u32 - 96
        }
        i += 3;
    }
    println!("{}", score);
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
