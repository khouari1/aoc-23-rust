advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let sum = input.lines().map(|line| {
        let numbers: Vec<i64> = get_numbers(line);
        run(numbers)
    }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sum = input.lines().map(|line| {
        let mut numbers: Vec<i64> = get_numbers(line);
        numbers.reverse();
        run(numbers)
    }).sum();
    Some(sum)
}

fn get_numbers(line: &str) -> Vec<i64> {
    line.split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn run(numbers: Vec<i64>) -> i64 {
    let mut record: Vec<Vec<i64>> = Vec::new();
    let mut next: Vec<i64> = numbers.clone();
    record.push(numbers);
    loop {
        let mut new_seq: Vec<i64> = Vec::new();
        for i in 1..next.len() {
            let n0 = next.get(i - 1).unwrap();
            let n1 = next.get(i).unwrap();
            let result = n1 - n0;
            new_seq.push(result);
        }
        record.push(new_seq.clone());
        next = new_seq;
        if next.iter().filter(|n| **n != 0).count() == 0 {
            break;
        }
    }
    let mut last_result = 0;
    record.reverse();
    let mut prev_last = 0;
    for i in 1..record.len() {
        let last_element = record.get(i).unwrap().last().unwrap();
        let result = last_element + prev_last;
        prev_last = result;
        last_result = result;
    }
    last_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
