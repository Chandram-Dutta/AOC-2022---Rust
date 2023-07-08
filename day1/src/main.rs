use std::fs;

fn main() {
    let contents = read_line("input.txt");
    let mut stars: Vec<usize> = Vec::new();
    let mut highest_elves = [0, 0, 0];
    for line in contents {
        if line.is_empty() {
            let mut count = 0;
            for star in &stars {
                count += star;
            }
            if count > highest_elves[0] {
                highest_elves[0] = count;
            }
            bubble_sort(&mut highest_elves);
            stars = Vec::new()
        } else {
            let star: usize = line.parse().expect("Error Converting");
            stars.push(star)
        }
    }
    println!("{}", highest_elves[0] + highest_elves[1] + highest_elves[2]);
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn bubble_sort(vec: &mut [usize; 3]) -> &[usize; 3] {
    for i in 0..vec.len() {
        for j in 0..i {
            if vec[j] > vec[j + 1] {
                vec.swap(j + 1, j);
            }
        }
    }
    vec
}
