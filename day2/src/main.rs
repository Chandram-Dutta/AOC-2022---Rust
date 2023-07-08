use std::fs;

// Part 1
// A, X = Rock
// B, Y = Paper
// C, Z = Scissor

// Part 2
// X = Lose
// Y = Draw
// Z = Win

fn main() {
    let contents = read_line("input");
    let mut score = 0;
    for lines in contents {
        let (mut opp, mut me) = lines.split_at(1);
        opp = opp.trim();
        me = me.trim();
        match me {
            "X" => {
                // score += 1;
                match opp {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => {}
                }
            }
            "Y" => {
                score += 3;
                match opp {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => {}
                }
            }
            "Z" => {
                score += 6;
                match opp {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    println!("{}", score)
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
