use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut final_sum = 0;
    for (_, [first_operand, second_operand]) in regex.captures_iter(input).map(|c| c.extract()) {
        final_sum += first_operand.parse::<i32>().unwrap() * second_operand.parse::<i32>().unwrap();
    }

    Some(final_sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
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

    Some(sum.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
