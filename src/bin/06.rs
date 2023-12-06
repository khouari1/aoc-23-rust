use std::iter::zip;
advent_of_code::solution!(6);

#[derive(Debug)]
struct RaceData {
    time: usize,
    distance: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let race_datas: Vec<RaceData> = get_race_data_pt1(input);
    let result = race_datas
        .iter()
        .map(|rd| {
            let mut beats_record = 0;
            for hold_button in 0..rd.time {
                let distance_travelled = (rd.time - hold_button) * hold_button;
                if distance_travelled > rd.distance {
                    beats_record += 1;
                }
            }
            beats_record
        })
        .reduce(|a, b| a * b)
        .unwrap();
    Some(result)
}

fn get_race_data_pt1(input: &str) -> Vec<RaceData> {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let times: Vec<usize> = get_numbers_pt1(lines.get(0).unwrap());
    let distances: Vec<usize> = get_numbers_pt1(lines.get(1).unwrap());
    zip(times, distances)
        .map(|(a, b)| RaceData {
            time: a,
            distance: b,
        })
        .collect()
}

fn get_numbers_pt1(line: &str) -> Vec<usize> {
    line.split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let race_data = get_race_data_pt2(input);
    let mut beats_record = 0;
    for hold_button in 0..race_data.time {
        let distance_travelled = (race_data.time - hold_button) * hold_button;
        if distance_travelled > race_data.distance {
            beats_record += 1;
        }
    }
    Some(beats_record)
}

fn get_race_data_pt2(input: &str) -> RaceData {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let time: usize = get_numbers_pt2(lines.get(0).unwrap());
    let distance: usize = get_numbers_pt2(lines.get(1).unwrap());
    RaceData { time, distance }
}

fn get_numbers_pt2(line: &str) -> usize {
    line.split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
