advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let line_str = line.to_string();
            let dummy_vec: Vec<&str> = Vec::new();
            let dummy_fn: fn(&str) -> Option<u32> = |_| None;
            let first_digit: u32 =
                get_num(&line_str, &dummy_vec, dummy_fn).expect("first digit not found");
            let last_digit: u32 = get_num(&line_str.chars().rev().collect(), &dummy_vec, dummy_fn)
                .expect("last digit not found");
            (first_digit * 10) + last_digit
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_strings = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let number_strings_rev = vec![
        "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    let sum = input
        .lines()
        .map(|line| {
            let first_digit: u32 = get_num(&line.to_string(), &number_strings, num_frm_str)
                .expect("first digit not found");
            let last_digit: u32 = get_num(
                &line.chars().rev().collect(),
                &number_strings_rev,
                num_frm_str_rev,
            )
            .expect("last digit not found");
            (first_digit * 10) + last_digit
        })
        .sum();
    Some(sum)
}

fn get_num(
    line: &String,
    number_strings: &Vec<&str>,
    num_frm_str: fn(&str) -> Option<u32>,
) -> Option<u32> {
    let mut digit: Option<u32> = None;
    'outer: for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_digit(10) {
            digit = Some(c.to_digit(10).unwrap());
            break;
        }
        let remaining_letters_length = line.len() - i;
        for n in number_strings {
            if n.len() <= remaining_letters_length {
                if line[i..i + n.len()].to_lowercase() == n.to_string() {
                    digit = Some(num_frm_str(n).unwrap());
                    break 'outer;
                }
            }
        }
    }
    digit
}

fn num_frm_str(s: &str) -> Option<u32> {
    match s {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn num_frm_str_rev(s: &str) -> Option<u32> {
    match s {
        "orez" => Some(0),
        "eno" => Some(1),
        "owt" => Some(2),
        "eerht" => Some(3),
        "ruof" => Some(4),
        "evif" => Some(5),
        "xis" => Some(6),
        "neves" => Some(7),
        "thgie" => Some(8),
        "enin" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
