advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let record = parts.get(0).unwrap().split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let numbers = parts.get(1).unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        find_arrangements(&numbers, 0, 0, &record)
    }).sum();
    Some(sum)
}

fn find_arrangements(numbers: &Vec<usize>, index_in_record: usize, number_index: usize, record: &Vec<&str>) -> usize {
    if number_index == numbers.len() {
        // check no more hash in rest of record
        for i in index_in_record..record.len() {
            let point = *record.get(i).unwrap();
            if point == "#" {
                // hash found, invalid arrangement
                return 0;
            }
        }
        // if no more numbers to check then valid arrangement
        return 1;
    }
    // at current position, can number fit (going forward)
    let number = numbers.get(number_index).unwrap();
    if index_in_record + number > record.len() {
        // unsuccessful arrangement and no more space
        return 0;
    }
    let mut number_fit = true;
    let mut first_n_hash = *record.get(index_in_record).unwrap() == "#";
    for record_pos in index_in_record..index_in_record + number {
        let point = *record.get(record_pos).unwrap();
        match point {
            "#" | "?" => {
                // good
            }
            _ => {
                // bad
                number_fit = false;
            }
        }
    }
    return if !number_fit {
        // unsuccessful arrangement
        // if first position was hash though then invalid arrangement
        if first_n_hash {
            0
        } else {
            // otherwise move onto next position
            find_arrangements(numbers, index_in_record + 1, number_index, &record.clone())
        }
    } else {
        // number successfully placed
        if index_in_record + number < record.len() && *record.get(index_in_record + number).unwrap() == "#" {
            // next position is hash so invalid arrangement
            // if first position was hash though then invalid arrangement
            if first_n_hash {
                0
            } else {
                // else move onto next position
                let mut cloned_record = record.clone();
                if *record.get(index_in_record).unwrap() == "?" {
                    cloned_record[index_in_record] = ".";
                }
                find_arrangements(numbers, index_in_record + 1, number_index, &cloned_record)
            }
        } else if index_in_record > 0 && *record.get(index_in_record - 1).unwrap() == "#" {
            // previous is hash, skip to next
            let mut cloned_record = record.clone();
            if *record.get(index_in_record).unwrap() == "?" {
                cloned_record[index_in_record] = ".";
            }
            find_arrangements(numbers, index_in_record + 1, number_index, &cloned_record)
        } else {
            let mut cloned_record = record.clone();
            for record_pos in index_in_record..index_in_record + number {
                cloned_record[record_pos] = "#";
            }
            let success_path = find_arrangements(numbers, index_in_record + number + 1, number_index + 1, &cloned_record);
            let mut dot_cloned_record = record.clone();
            if *record.get(index_in_record).unwrap() == "?" {
                dot_cloned_record[index_in_record] = ".";
            }
            // only do dot path if point is ?
            let dot_path = if *record.get(index_in_record).unwrap() == "?" {
                find_arrangements(numbers, index_in_record + 1, number_index, &dot_cloned_record)
            } else {
                0
            };
            success_path + dot_path
        }
    };
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let mut record = parts.get(0).unwrap().split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let mut numbers = parts.get(1).unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let mut unfolded_record: Vec<&str> = Vec::new();
        let mut unfolded_numbers: Vec<usize> = Vec::new();
        for i in 0..5 {
            unfolded_record.append(&mut record.clone());
            if i < 4 {
                unfolded_record.push("?");
            }
            unfolded_numbers.append(&mut numbers.clone());
        }

        // println!("{:?}", unfolded_record);
        find_arrangements(&unfolded_numbers, 0, 0, &unfolded_record)
    }).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
