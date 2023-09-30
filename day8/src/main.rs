use std::fs;

fn main() {
    let contents = read_line("input.txt");
    let mut forest: Vec<Vec<usize>> = vec![];
    for lines in contents {
        let row: Vec<usize> = lines
            .chars()
            .map(|c| c.to_string().parse().expect("Error Parsing"))
            .collect();
        forest.push(row);
    }
    let mut high = 0;
    // let mut visible = (forest.len() * 2) + ((forest.len() - 2) * 2);
    // println!("{}", visible);
    // let mut visible = 0;

    for row_no in 1..forest.len() - 1 {
        for tree_no in 1..forest[row_no].len() - 1 {
            let tree = forest[row_no][tree_no];
            let mut left = forest[row_no][0..tree_no].to_vec();
            left.reverse();
            let right = &forest[row_no][tree_no + 1..].to_vec();
            let mut top: Vec<usize> = vec![];
            for rows in &forest[0..row_no] {
                top.push(rows[tree_no]);
            }
            top.reverse();
            let mut bottom: Vec<usize> = vec![];
            for rows in &forest[row_no + 1..] {
                bottom.push(rows[tree_no]);
            }
            let r_ans = score(tree, right);
            let l_ans = score(tree, &left);
            let t_ans = score(tree, &top);
            let b_ans = score(tree, &bottom);
            let scenic_score = r_ans * l_ans * t_ans * b_ans;
            println!(
                "tree: {}, r: {}, l: {}, t: {}, b:{}, s: {}",
                tree, r_ans, l_ans, t_ans, b_ans, scenic_score
            );
            if scenic_score > high {
                high = scenic_score;
            }

            // if r_ans || l_ans || t_ans || b_ans {
            //     visible += 1
            // }
        }
    }

    println!("{}", high)

    // println!(
    //     "{}x{} trees => {} visible",
    //     forest.len(),
    //     forest[1].len(),
    //     visible
    // )
}

fn score(tree: usize, trees: &[usize]) -> usize {
    let mut counter = 0;
    for i in 0..trees.len() {
        counter += 1;
        if tree <= trees[i] {
            return counter;
        }
    }
    counter
}

fn read_line(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
