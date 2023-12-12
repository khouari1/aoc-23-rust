use std::collections::HashSet;
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 999_999)
}

fn solve(input: &str, diff: usize) -> Option<usize> {
    // record each row and column that have galaxies
    let mut galaxy_coords: Vec<(usize, usize)> = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut rows_with_galaxies: HashSet<usize> = HashSet::new();
    let mut cols_with_galaxies: HashSet<usize> = HashSet::new();
    for i in 0..lines.len() {
        let row: Vec<&str> = lines.get(i).unwrap().split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        for j in 0..row.len() {
            let col = *row.get(j).unwrap();
            if col == "#" {
                rows_with_galaxies.insert(i);
                cols_with_galaxies.insert(j);
                // record the coordinates of each galaxy
                galaxy_coords.push((j, i));
            }
        }
    }
    let mut sum: usize = 0;
    // for each galaxy
    let mut calced_paths: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for i in 0..galaxy_coords.len() {
        let (galaxy_x, galaxy_y) = galaxy_coords.get(i).unwrap();
        // for each other galaxy
        for j in 0..galaxy_coords.len() {
            if i == j {
                // ignore same galaxy
                continue;
            }
            let (other_x, other_y) = galaxy_coords.get(j).unwrap();
            if calced_paths.contains(&((*other_x, *other_y), (*galaxy_x, *galaxy_y))) {
                continue;
            }
            // find diff between coords
            let mut x_diff = (*galaxy_x as i64 - *other_x as i64).abs() as usize;
            let mut y_diff = (*galaxy_y as i64 - *other_y as i64).abs() as usize;
            // check if coords encompass rows or columns that have no galaxies
            // if so add the number to the diff
            let (first_x, second_x) = if galaxy_x > other_x { (other_x, galaxy_x) } else { (galaxy_x, other_x) };
            for x in *first_x..*second_x {
                if !cols_with_galaxies.contains(&x) {
                    x_diff += diff;
                }
            }
            let (first_y, second_y) = if galaxy_y > other_y { (other_y, galaxy_y) } else { (galaxy_y, other_y) };
            for y in *first_y..*second_y {
                if !rows_with_galaxies.contains(&y) {
                    y_diff += diff;
                }
            }
            // record path lengths
            let diff = x_diff + y_diff;
            sum += diff;
            calced_paths.insert(((*galaxy_x, *galaxy_y), (*other_x, *other_y)));
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
