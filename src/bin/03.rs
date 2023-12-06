use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    // create grid map
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let number_of_lines = lines.len();
    for i in 0..number_of_lines {
        let line: &String = lines.get(i).unwrap();
        // iterate until find digit
        let chars: Vec<char> = line.chars().collect();
        for j in 0..chars.len() {
            let c = chars.get(j).unwrap();
            grid.insert((j, i), *c);
        }
    }

    let mut total: usize = 0;
    // for each line
    for i in 0..number_of_lines {
        let line: &String = lines.get(i).unwrap();
        // iterate until find digit
        let mut in_number = false;
        let mut current_number = "".to_string();
        let mut found_symbol = false;
        let chars: Vec<char> = line.chars().collect();
        let number_of_chars = chars.len();
        for j in 0..number_of_chars {
            let c: &char = chars.get(j).unwrap();
            if c.is_digit(10) {
                if !in_number {
                    // start of new number
                    in_number = true;
                }
                current_number.push(*c);
                if found_symbol {
                    // no need to check for symbols around other digits, just get to end of number
                    continue;
                }
                // check around number for symbol
                // check above
                if i > 0 {
                    if check_c(&grid, j, i - 1) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check below
                if i < number_of_lines - 1 {
                    if check_c(&grid, j, i + 1) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check left
                if j > 0 {
                    if check_c(&grid, j - 1, i) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check right
                if j < number_of_chars - 1 {
                    if check_c(&grid, j + 1, i) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check top left
                if i > 0 && j > 0 {
                    if check_c(&grid, j - 1, i - 1) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check top right
                if i > 0 && j < number_of_chars - 1 {
                    if check_c(&grid, j + 1, i - 1) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check bottom left
                if i < number_of_lines - 1 && j > 0 {
                    if check_c(&grid, j - 1, i + 1) {
                        found_symbol = true;
                        continue;
                    }
                }
                // check bottom right
                if i < number_of_lines - 1 && j < number_of_chars - 1 {
                    if check_c(&grid, j + 1, i + 1) {
                        found_symbol = true;
                        continue;
                    }
                }
            } else if in_number {
                // reached end of number
                if found_symbol {
                    total += current_number.parse::<usize>().unwrap();
                }
                // reset state
                in_number = false;
                current_number = "".to_string();
                found_symbol = false;
            }
        }
        if found_symbol {
            total += current_number.parse::<usize>().unwrap();
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    // create grid map
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let number_of_lines = lines.len();
    for i in 0..number_of_lines {
        let line: &String = lines.get(i).unwrap();
        // iterate until find digit
        let chars: Vec<char> = line.chars().collect();
        for j in 0..chars.len() {
            let c = chars.get(j).unwrap();
            grid.insert((j, i), *c);
        }
    }

    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    // for each line
    for i in 0..number_of_lines {
        let line: &String = lines.get(i).unwrap();
        // iterate until find digit
        let mut in_number = false;
        let mut current_number = "".to_string();
        let mut found_symbol = false;
        let chars: Vec<char> = line.chars().collect();
        let number_of_chars = chars.len();
        let mut stars: HashSet<(usize, usize)> = HashSet::new();
        for j in 0..number_of_chars {
            let c: &char = chars.get(j).unwrap();
            if c.is_digit(10) {
                if !in_number {
                    // start of new number
                    in_number = true;
                }
                current_number.push(*c);
                if found_symbol {
                    // no need to check for symbols around other digits, just get to end of number
                    continue;
                }
                // check around number for symbol
                // check above
                if i > 0 {
                    if check_c(&grid, j, i - 1) {
                        let c_to_check = grid.get(&(j, i - 1)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j, i - 1));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check below
                if i < number_of_lines - 1 {
                    if check_c(&grid, j, i + 1) {
                        let c_to_check = grid.get(&(j, i + 1)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j, i + 1));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check left
                if j > 0 {
                    if check_c(&grid, j - 1, i) {
                        let c_to_check = grid.get(&(j - 1, i)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j - 1, i));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check right
                if j < number_of_chars - 1 {
                    if check_c(&grid, j + 1, i) {
                        let c_to_check = grid.get(&(j + 1, i)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j + 1, i));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check top left
                if i > 0 && j > 0 {
                    if check_c(&grid, j - 1, i - 1) {
                        let c_to_check = grid.get(&(j - 1, i - 1)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j - 1, i - 1));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check top right
                if i > 0 && j < number_of_chars - 1 {
                    if check_c(&grid, j + 1, i - 1) {
                        let c_to_check = grid.get(&(j + 1, i - 1)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j + 1, i - 1));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check bottom left
                if i < number_of_lines - 1 && j > 0 {
                    if check_c(&grid, j - 1, i + 1) {
                        let c_to_check = grid.get(&(j - 1, i + 1)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j - 1, i + 1));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
                // check bottom right
                if i < number_of_lines - 1 && j < number_of_chars - 1 {
                    if check_c(&grid, j + 1, i + 1) {
                        let c_to_check = grid.get(&(j + 1, i + 1)).unwrap();
                        if *c_to_check == '*' {
                            stars.insert((j + 1, i + 1));
                        }
                        found_symbol = true;
                        continue;
                    }
                }
            } else if in_number {
                if !stars.is_empty() {
                    stars.iter().for_each(|(x, y)| {
                        let mut v = gears.entry((*x, *y)).or_default();
                        v.push(current_number.parse::<usize>().unwrap());
                    });
                }
                // reset state
                in_number = false;
                current_number = "".to_string();
                found_symbol = false;
                stars.clear();
            }
        }
        if !stars.is_empty() {
            stars.iter().for_each(|(x, y)| {
                let mut v = gears.entry((*x, *y)).or_default();
                v.push(current_number.parse::<usize>().unwrap());
            });
        }
    }
    let total = gears
        .iter()
        .filter(|(k, v)| v.len() > 1)
        .map(|(k, v)| v.clone().into_iter().reduce(|a, b| a * b))
        .map(|x| x.unwrap())
        .sum();
    Some(total)
}

fn check_c(grid: &HashMap<(usize, usize), char>, j: usize, i: usize) -> bool {
    let coord: (usize, usize) = (j, i);
    let c_to_check = grid.get(&coord).unwrap();
    !c_to_check.is_digit(10) && *c_to_check != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
