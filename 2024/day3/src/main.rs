use regex::Regex;
use std::fs;

fn solve_task_1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut final_sum = 0;
    for (_, [first_operand, second_operand]) in regex.captures_iter(input).map(|c| c.extract()) {
        final_sum += first_operand.parse::<i32>().unwrap() * second_operand.parse::<i32>().unwrap();
    }

    final_sum
}

fn solve_task_2(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|(do\(\))").unwrap();

    let mut sum = 0;
    let mut skip_next = false;

    for capture in regex.captures_iter(input) {
        let (operand1, operand2) = (capture.get(1).ok_or(""), capture.get(2).ok_or(""));

        // apparently this is not supported yet
        // if let Some(operand1) = capture.get(1) && let Some(operand2) = capture.get(2) {
        if operand1.is_ok() && operand2.is_ok() {
            if skip_next {
                continue;
            }
            sum += operand1.unwrap().as_str().parse::<i32>().unwrap()
                * operand2.unwrap().as_str().parse::<i32>().unwrap();
        } else if capture.get(3).is_some() {
            skip_next = true;
        } else if capture.get(4).is_some() {
            skip_next = false;
        }
    }

    sum
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
