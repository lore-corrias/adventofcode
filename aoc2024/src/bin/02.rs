advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
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

    Some(safe_reports_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
