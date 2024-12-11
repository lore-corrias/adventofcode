use std::fs;

fn solve_task_1(input: &str) -> i32 {
    let reports: Vec<&str> = input.lines().collect();
    let mut safe_reports_count = 0;

    for report in reports {
        let levels: Vec<i32> = report
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut report_safe = true;

        // Determining the order
        let order = levels[0].cmp(&levels[1]);
        for j in 0..levels.len() - 1 {
            let levels_difference = (levels[j] - levels[j + 1]).abs();
            if !(1..=3).contains(&levels_difference) || levels[j].cmp(&levels[j + 1]) != order {
                report_safe = false;
                break;
            }
        }

        if report_safe {
            safe_reports_count += 1;
        }
    }

    safe_reports_count
}

fn solve_task_2(input: &str) -> i32 {}

fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't find first input file!");

    println!(
        "The solution for the first task is: {}",
        solve_task_1(&input)
    );
    println!(
        "The solution for the second task is: {}",
        solve_task_2(&input)
    );
}
