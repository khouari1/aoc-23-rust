use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let red_to_find = 12;
    let green_to_find = 13;
    let blue_to_find = 14;
    let mut valid_games: HashSet<u32> = HashSet::new();
    input.lines().for_each(|line| {
        let (game, reveals) = get_game_info(&line.to_string());
        let mut red_too_high = false;
        let mut green_too_high = false;
        let mut blue_too_high = false;
        calc_game(
            reveals,
            |n| {
                if n > &red_to_find {
                    red_too_high = true;
                }
            },
            |n| {
                if n > &green_to_find {
                    green_too_high = true;
                }
            },
            |n| {
                if n > &blue_to_find {
                    blue_too_high = true;
                }
            },
        );
        if !red_too_high && !green_too_high && !blue_too_high {
            valid_games.insert(game);
        }
    });
    Some(valid_games.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut powers: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let (_, reveals) = get_game_info(&line.to_string());
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        calc_game(
            reveals,
            |n| {
                if n > &min_red {
                    min_red = *n;
                }
            },
            |n| {
                if n > &min_green {
                    min_green = *n;
                }
            },
            |n| {
                if n > &min_blue {
                    min_blue = *n;
                }
            },
        );
        powers.push(min_red * min_green * min_blue);
    });
    Some(powers.iter().sum())
}

fn get_game_info(line: &String) -> (u32, Vec<String>) {
    let mut parts = line.split(":");
    let game: u32 = parts
        .next()
        .unwrap()
        .replace("Game ", "")
        .parse::<u32>()
        .unwrap();
    let reveals: Vec<String> = parts
        .next()
        .unwrap()
        .split(";")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    (game, reveals)
}

fn calc_game(
    reveals: Vec<String>,
    mut red_fn: impl FnMut(&u32) -> (),
    mut green_fn: impl FnMut(&u32) -> (),
    mut blue_fn: impl FnMut(&u32) -> (),
) {
    reveals.iter().for_each(|r| {
        r.split(",")
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|part| {
                let mut part_split = part.split(" ");
                let n = &part_split.next().unwrap().parse::<u32>().unwrap();
                let colour = &part_split.next().unwrap().parse::<String>().unwrap();
                match colour.as_str() {
                    "red" => {
                        red_fn(n);
                    }
                    "green" => {
                        green_fn(n);
                    }
                    "blue" => {
                        blue_fn(n);
                    }
                    _ => {}
                }
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
