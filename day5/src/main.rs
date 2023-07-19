use std::{collections::HashMap, fs, vec};

fn main() {
    let one = vec!["S", "Z", "P", "D", "L", "B", "F", "C"];
    let two = vec!["N", "V", "G", "P", "H", "W", "B"];
    let three = vec!["F", "W", "B", "J", "G"];
    let four = vec!["G", "J", "N", "F", "L", "W", "C", "S"];
    let five = vec!["W", "J", "L", "T", "P", "M", "S", "H"];
    let six = vec!["B", "C", "W", "G", "F", "S"];
    let seven = vec!["H", "T", "P", "M", "Q", "B", "W"];
    let eight = vec!["F", "S", "W", "T"];
    let nine = vec!["N", "C", "R"];

    let mut map = HashMap::from([
        (1, one),
        (2, two),
        (3, three),
        (4, four),
        (5, five),
        (6, six),
        (7, seven),
        (8, eight),
        (9, nine),
    ]);

    let contents = read_line("input1.txt");

    for lines in contents {
        let split: Vec<&str> = lines.split_whitespace().collect();
        let mut elements: Vec<&str> = vec![];
        for _ in 0..split[1].parse::<i32>().expect("") {
            let p = map
                .get_mut(&split[3].parse::<i32>().expect(""))
                .expect("")
                .pop()
                .expect("");

            elements.push(p);
        }
        elements.reverse();
        map.get_mut(&split[5].parse::<i32>().expect(""))
            .expect("")
            .append(&mut elements);
    }
    println!(
        "{}{}{}{}{}{}{}{}{}",
        map.get(&1).expect("").last().expect(""),
        map.get(&2).expect("").last().expect(""),
        map.get(&3).expect("").last().expect(""),
        map.get(&4).expect("").last().expect(""),
        map.get(&5).expect("").last().expect(""),
        map.get(&6).expect("").last().expect(""),
        map.get(&7).expect("").last().expect(""),
        map.get(&8).expect("").last().expect(""),
        map.get(&9).expect("").last().expect("")
    )
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
