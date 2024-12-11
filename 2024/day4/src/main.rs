use std::fs;

fn solve_task_1(input: &str) -> i32 {
    let symbols: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count = 0;
    let xmas: Vec<char> = "XMAS".chars().collect();

    let directions: [(i32, i32); 9] = [
        // downwards
        (-1, -1),
        (-1, 0),
        (-1, 1),
        // horizontally
        (0, -1),
        (0, 0),
        (0, 1),
        // upwards
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..symbols.len() {
        for j in 0..symbols[i].len() {
            if symbols[i][j] == 'X' {
                for direction in directions {
                    let (x, y) = direction;

                    // checking if we have enough space to move
                    if ((j as i32 + 3 * x) >= 0 && (j as i32 + 3 * x) < symbols[i].len() as i32)
                        && ((i as i32 + 3 * y) >= 0 && (i as i32 + 3 * y) < symbols.len() as i32)
                    {
                        let mut current_char = 1;
                        for l in 1..4 {
                            if symbols[(i as i32 + l * y) as usize][(j as i32 + l * x) as usize]
                                == xmas[current_char]
                            {
                                current_char += 1;
                            } else {
                                break;
                            }
                        }
                        if current_char == 4 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn solve_task_2(input: &str) -> i32 {
    let symbols: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count = 0;

    for i in 0..symbols.len() {
        for j in 0..symbols[i].len() {
            if symbols[i][j] == 'A'
                && i >= 1
                && j >= 1
                && i < symbols.len() - 1
                && j < symbols[i].len() - 1
            {
                count += (((symbols[i - 1][j - 1] == 'M' && symbols[i + 1][j + 1] == 'S')
                    || (symbols[i - 1][j - 1] == 'S' && symbols[i + 1][j + 1] == 'M'))
                    && ((symbols[i + 1][j - 1] == 'S' && symbols[i - 1][j + 1] == 'M')
                        || (symbols[i + 1][j - 1] == 'M' && symbols[i - 1][j + 1] == 'S')))
                    as i32;
            }
        }
    }

    count
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
