use std::collections::HashMap;
use num::integer::gcd;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let directions: Vec<&str> = lines.get(0).unwrap().split("").filter(|s| !s.is_empty()).collect();
    let mut maps: HashMap<&str, (&str, &str)> = HashMap::new();
    lines.drain(0..2);
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts.get(0).unwrap().clone();
        let dir_parts = parts.get(1).unwrap().split(", ").collect::<Vec<&str>>();
        let mut left_chars = dir_parts.get(0).unwrap().chars();
        left_chars.next();
        let left: &str = left_chars.as_str();
        let mut right_chars = dir_parts.get(1).unwrap().chars();
        right_chars.next_back();
        let right: &str = right_chars.as_str();
        maps.insert(key, (left, right));
    });
    let mut steps = 0;
    let mut current_pos = "AAA";
    let mut i = 0;
    loop {
        let direction: &str = directions.get(i).unwrap();
        if current_pos == "ZZZ" {
            break;
        }
        let (left, right) = maps.get(current_pos).unwrap();
        current_pos = match direction {
            "L" => left,
            "R" => right,
            _ => panic!("Could not match direction {}", direction),
        };
        steps += 1;
        if i == directions.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let directions: Vec<&str> = lines.get(0).unwrap().split("").filter(|s| !s.is_empty()).collect();
    let mut maps: HashMap<&str, (&str, &str)> = HashMap::new();
    lines.drain(0..2);
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts.get(0).unwrap().clone();
        let dir_parts = parts.get(1).unwrap().split(", ").collect::<Vec<&str>>();
        let mut left_chars = dir_parts.get(0).unwrap().chars();
        left_chars.next();
        let left: &str = left_chars.as_str();
        let mut right_chars = dir_parts.get(1).unwrap().chars();
        right_chars.next_back();
        let right: &str = right_chars.as_str();
        maps.insert(key, (left, right));
    });
    let mut loop_positions = Vec::new();
    let mut original_positions = Vec::new();
    let mut current_positions = Vec::new();
    maps.iter().for_each(|(k, _)| {
        if k.ends_with("A") {
            current_positions.push(*k);
            original_positions.push(*k);
        }
    });
    for j in 0..current_positions.len() {
        let mut i = 0;
        let mut steps = 0;
        loop {
            let current_pos = current_positions.get(j).unwrap().clone();
            let direction: &str = directions.get(i).unwrap();
            let (left, right) = maps.get(current_pos).unwrap();
            let new_pos = match direction {
                "L" => left,
                "R" => right,
                _ => panic!("Could not match direction {}", direction),
            };
            current_positions[j] = &new_pos;
            steps += 1;
            if new_pos.ends_with("Z") {
                loop_positions.push(steps);
                break;
            }
            if i == directions.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }
    }
    let result = lcm_list(&loop_positions);
    Some(result)
}

fn lcm_list(numbers: &Vec<usize>) -> usize {
    numbers.iter().fold(1, |a, b| lcm(a, *b))
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
