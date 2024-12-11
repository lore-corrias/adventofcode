use std::fs;

fn solve_task_1(input: &str) -> i32 {
    let symbols: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count = 0;
    let xmas = "XMAS".chars()
    let samx = xmas.reverse();

    for i in 0..symbols.len() {
        for j in 0..symbols[i].len() {
            if symbols[i][j] == 'X' {
                // searches SAMX
                if j >= 4 {
                    count += (symbols[i][j..j - 4] == ['S', 'A', 'M', 'X']) as u32;
                // searches XMAS
                } else if j < symbols[i].len() - 4 {
                    count += (symbols[i][j..j + 4] == ['X', 'M', 'A', 'S']) as u32;
                // searches SAMX vertically
                } else if i >= 4 {
                    count +=
                        (symbols[i..i - 4].iter().map(|c| c[j]) == ['S', 'A', 'M', 'X']) as u32;
                }
            }
        }
    }

    1
}

fn solve_task_2(input: &str) -> i32 {
    0
}

fn main() {
    let input = fs::read_to_string("./input").expect("Could not read input file.");

    println!(
        "The solution for the first task is: {}",
        solve_task_1(&input)
    );
    println!(
        "The solution for the second task is: {}",
        solve_task_2(&input)
    );
}
