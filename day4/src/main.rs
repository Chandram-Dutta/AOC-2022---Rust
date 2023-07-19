use std::fs;

fn main() {
    let mut count = 0;
    let contents = read_line("input.txt");
    for lines in contents {
        let split = lines.find(",").expect("err");
        let (first, second) = (&lines[0..split], &lines[(split + 1)..]);
        let first = first.replace("-", " ");
        let first: Vec<&str> = first.split_whitespace().collect();
        let first: [usize; 2] = [first[0].parse().unwrap(), first[1].parse().unwrap()];
        let second = second.replace("-", " ");
        let second: Vec<&str> = second.split_whitespace().collect();
        let second: [usize; 2] = [second[0].parse().unwrap(), second[1].parse().unwrap()];
        if (first[1] >= second[0] && first[0] <= second[1])
            || (second[1] >= first[0] && second[0] <= first[1])
        {
            count += 1;
        }
    }
    println!("{}", count)
}
fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
