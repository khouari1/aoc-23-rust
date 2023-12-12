use std::collections::{HashMap, HashSet};
use std::hash::Hash;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let mut data_by_coord: HashMap<(usize, usize), &str> = HashMap::new();
    let rows = input.lines().collect::<Vec<&str>>();
    let mut starting_point: (usize, usize) = (0, 0);
    let mut col_len = 0;
    for i in 0..rows.len() {
        let row = rows.get(i).unwrap();
        let cols = row.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let col_length = cols.len();
        col_len = col_length;
        for j in 0..col_len {
            let col = *cols.get(j).unwrap();
            let coord = (j, i);
            if col == "S" {
                starting_point = coord;
            }
            data_by_coord.insert(coord, col);
        }
    }
    let mut steps: usize = 0;
    let mut pipe_coord = (0, 0);
    let mut found_pipe = false;
    let starting_x = starting_point.0;
    let starting_y = starting_point.1;
    if starting_y > 0 {
        // check above
        let point = data_by_coord.get(&(starting_x, starting_y - 1)).unwrap();
        if is_pipe("D", point) {
            pipe_coord = (starting_x, starting_y - 1);
            found_pipe = true;
        }
    }
    if starting_y < rows.len() - 2 && !found_pipe {
        // check below
        let point = data_by_coord.get(&(starting_x, starting_y + 1)).unwrap();
        if is_pipe("U", point) {
            pipe_coord = (starting_x, starting_y + 1);
            found_pipe = true;
        }
    }
    if starting_x > 0 && !found_pipe {
        // check left
        let point = data_by_coord.get(&(starting_x - 1, starting_y)).unwrap();
        if is_pipe("R", point) {
            pipe_coord = (starting_x - 1, starting_y);
            found_pipe = true;
        }
    }
    if starting_x < col_len - 2 && !found_pipe {
        // check right
        let point = data_by_coord.get(&(starting_x + 1, starting_y)).unwrap();
        if is_pipe("L", point) {
            pipe_coord = (starting_x + 1, starting_y);
            found_pipe = true;
        }
    }
    let mut previous_pipe_coord = starting_point;
    loop {
        steps += 1;
        let mut current_x = pipe_coord.0;
        let mut current_y = pipe_coord.1;
        let mut prev_x = previous_pipe_coord.0;
        let mut prev_y = previous_pipe_coord.1;
        let current_pipe = *data_by_coord.get(&pipe_coord).unwrap();
        previous_pipe_coord = pipe_coord;
        match current_pipe {
            "|" => {
                if prev_y < current_y {
                    // go up
                    pipe_coord = (current_x, current_y + 1);
                } else {
                    // go down
                    pipe_coord = (current_x, current_y - 1);
                }
            }
            "-" => {
                if prev_x < current_x {
                    // go right
                    pipe_coord = (current_x + 1, current_y);
                } else {
                    // go left
                    pipe_coord = (current_x - 1, current_y);
                }
            }
            "L" => {
                if prev_y < current_y {
                    // go right
                    pipe_coord = (current_x + 1, current_y);
                } else {
                    // go up
                    pipe_coord = (current_x, current_y - 1);
                }
            }
            "J" => {
                if prev_y < current_y {
                    // go left
                    pipe_coord = (current_x - 1, current_y);
                } else {
                    // go up
                    pipe_coord = (current_x, current_y - 1);
                }
            }
            "7" => {
                if prev_y > current_y {
                    // go left
                    pipe_coord = (current_x - 1, current_y);
                } else {
                    // go down
                    pipe_coord = (current_x, current_y + 1);
                }
            }
            "F" => {
                if prev_y > current_y {
                    // go right
                    pipe_coord = (current_x + 1, current_y);
                } else {
                    // go down
                    pipe_coord = (current_x, current_y + 1);
                }
            }
            "S" => {
                println!("Hit starting point again!");
                break;
            }
            _ => panic!("Unexpected"),
        }
    }
    Some(steps / 2)
}

fn is_pipe(starting_point_in_relation_to: &str, point: &str) -> bool {
    match point {
        "|" => starting_point_in_relation_to == "U" || starting_point_in_relation_to == "D",
        "-" => starting_point_in_relation_to == "L" || starting_point_in_relation_to == "R",
        "L" => starting_point_in_relation_to == "U" || starting_point_in_relation_to == "R",
        "J" => starting_point_in_relation_to == "U" || starting_point_in_relation_to == "L",
        "7" => starting_point_in_relation_to == "D" || starting_point_in_relation_to == "L",
        "F" => starting_point_in_relation_to == "D" || starting_point_in_relation_to == "R",
        _ => false,
    }
}

fn is_pipe2(point: &str) -> bool {
    match point {
        "|" |
        "-" |
        "L" |
        "J" |
        "7" |
        "F" => true,
        _ => false,
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut data_by_coord: HashMap<(usize, usize), &str> = HashMap::new();
    let rows = input.lines().collect::<Vec<&str>>();
    let mut starting_point: (usize, usize) = (0, 0);
    let mut col_len = 0;
    for i in 0..rows.len() {
        let row = rows.get(i).unwrap();
        let cols = row.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let col_length = cols.len();
        col_len = col_length;
        for j in 0..col_len {
            let col = *cols.get(j).unwrap();
            let coord = (j, i);
            if col == "S" {
                starting_point = coord;
            }
            data_by_coord.insert(coord, col);
        }
    }
    let mut steps: usize = 0;
    let mut pipe_coord = (0, 0);
    let mut found_pipe = false;
    let starting_x = starting_point.0;
    let starting_y = starting_point.1;
    let mut loop_coords: HashMap<usize, Vec<usize>> = HashMap::new();
    loop_coords.entry(starting_y).or_default().push(starting_x);
    println!("Starting = {:?}", starting_point);
    println!("loop_coords = {:?}", loop_coords);
    if starting_y > 0 {
        // check above
        let point = data_by_coord.get(&(starting_x, starting_y - 1)).unwrap();
        if is_pipe("D", point) {
            pipe_coord = (starting_x, starting_y - 1);
            found_pipe = true;
        }
    }
    if starting_y < rows.len() - 2 && !found_pipe {
        // check below
        let point = data_by_coord.get(&(starting_x, starting_y + 1)).unwrap();
        if is_pipe("U", point) {
            pipe_coord = (starting_x, starting_y + 1);
            found_pipe = true;
        }
    }
    if starting_x > 0 && !found_pipe {
        // check left
        let point = data_by_coord.get(&(starting_x - 1, starting_y)).unwrap();
        if is_pipe("R", point) {
            pipe_coord = (starting_x - 1, starting_y);
            found_pipe = true;
        }
    }
    if starting_x < col_len - 2 && !found_pipe {
        // check right
        let point = data_by_coord.get(&(starting_x + 1, starting_y)).unwrap();
        if is_pipe("L", point) {
            pipe_coord = (starting_x + 1, starting_y);
            found_pipe = true;
        }
    }
    let mut previous_pipe_coord = starting_point;
    loop {
        steps += 1;
        let mut current_x = pipe_coord.0;
        let mut current_y = pipe_coord.1;
        let mut prev_x = previous_pipe_coord.0;
        let mut prev_y = previous_pipe_coord.1;
        let current_pipe = *data_by_coord.get(&pipe_coord).unwrap();
        let set = loop_coords.entry(pipe_coord.1).or_default();
        set.push(pipe_coord.0);
        previous_pipe_coord = pipe_coord;
        // println!("Pipe coord = {:?}", pipe_coord);
        let mut actual_s = "";
        match current_pipe {
            "|" => {
                if prev_y < current_y {
                    // go up
                    pipe_coord = (current_x, current_y + 1);
                } else {
                    // go down
                    pipe_coord = (current_x, current_y - 1);
                }
            }
            "-" => {
                if prev_x < current_x {
                    // go right
                    pipe_coord = (current_x + 1, current_y);
                } else {
                    // go left
                    pipe_coord = (current_x - 1, current_y);
                }
            }
            "L" => {
                if prev_y < current_y {
                    // go right
                    pipe_coord = (current_x + 1, current_y);
                } else {
                    // go up
                    pipe_coord = (current_x, current_y - 1);
                }
            }
            "J" => {
                if prev_y < current_y {
                    // go left
                    pipe_coord = (current_x - 1, current_y);
                } else {
                    // go up
                    pipe_coord = (current_x, current_y - 1);
                }
            }
            "7" => {
                if prev_y > current_y {
                    // go left
                    pipe_coord = (current_x - 1, current_y);
                } else {
                    // go down
                    pipe_coord = (current_x, current_y + 1);
                }
            }
            "F" => {
                if prev_y > current_y {
                    // go right
                    pipe_coord = (current_x + 1, current_y);
                } else {
                    // go down
                    pipe_coord = (current_x, current_y + 1);
                }
            }
            "S" => {
                println!("Hit starting point again!");
                break;
            }
            _ => panic!("Unexpected"),
        }
    }
    // for each space
    // check above, below, left and right
    // if, in all directions there is a loop pipe before any other pipe, it is enclosed
    let mut count = 0;
    for y in 0..rows.len() {
        // println!("Row {} loop pipes {:?}",  y, loop_coords.get(&y).unwrap());
        let mut in_pipe = false;
        let mut temp = 0;
        for x in 0..col_len {
            let mut dashes = 0;
            // let point = *data_by_coord.get(&(x, y)).unwrap();
            let option = loop_coords.get(&y);
            let is_loop_pipe = option.is_some() && loop_coords.get(&y).unwrap().contains(&x) || option.is_none();
            if is_loop_pipe {
                continue;
            }
            let mut hits = 0;
            for i in (x + 1)..col_len {
                let is_loop_pipe = option.unwrap().contains(&i);
                let point = *data_by_coord.get(&(i, y)).unwrap();
                // TODO: find out what S is, if J or L then ignore
                if is_loop_pipe && point != "-" && point != "J" && point != "L" && point != "S" {
                    // println!("Row {} loop pipe hit {}",  y, i);
                    hits += 1;
                }
            }
            if hits % 2 != 0 {
                // println!("Coord {:?}", &(x, y));
                // is inside
                count += 1;
            }
            // if is_loop_pipe && !in_pipe && point != "-" {
            //     in_pipe = true;
            // } else if is_loop_pipe && in_pipe && point != "-" {
            //     count += temp;
            //     if temp != 0 {
            //         println!("Row {}: {:?}", y, temp);
            //     }
            //     temp = 0;
            //     in_pipe = false;
            // } else if !is_loop_pipe && in_pipe {
            //     println!("Counting {:?}", (x, y));
            //     temp += 1;
            // }
        }
    }
    println!("Loop coords: {:?}", loop_coords);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(10));
    }
}
