use std::{cmp::Ordering, collections::{HashMap, HashSet}};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<u64, HashSet<u64>>, Vec<&str>){
    let elements: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
 
    // Generation of rules
    for rule in elements[0].lines() {
        let split_rule: Vec<u64> = rule.split("|").collect::<Vec<&str>>().iter().map(|x| x.parse::<u64>().unwrap()).collect();
        rules.entry(split_rule[0]).and_modify(|x| { x.insert(split_rule[1]); }).or_insert(HashSet::from_iter(vec![split_rule[1]].iter().cloned()));
    }

    (rules, elements[1].lines().collect::<Vec<&str>>())
}


pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;
 
    // Validation of order
    for update in updates {
        let pages = update.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let mut valid_order = true;
        let mut met_numbers: HashSet<u64> = HashSet::new();

        for page in pages.iter().rev() {
            if met_numbers.contains(page) {
                valid_order = false;
                break;
            }

            met_numbers.extend(rules.get(page).unwrap().iter());
        }

        if valid_order {
            sum += pages[pages.len() / 2];
        }
    }
 
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;


    for update in updates {
        let mut pages = update.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        
        pages.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if rules.get(a).expect("").contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        sum += pages[pages.len() / 2];
    }

    Some(sum)
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
