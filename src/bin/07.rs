use std::collections::HashMap;

use crate::PlayResult::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Play {
    hand: Vec<String>,
    bid: usize,
    result: PlayResult,
}

#[derive(Debug)]
enum PlayResult {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut plays: Vec<Play> = input.lines().map(|line| {
        let parts: Vec<String> = line.split(" ").map(|s| s.trim().to_string()).collect();
        let hand: Vec<String> = parts.get(0).unwrap().split("").map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        let result = get_result(&hand);
        Play {
            hand,
            bid: parts.get(1).unwrap().parse::<usize>().unwrap(),
            result,
        }
    }).collect();
    plays.sort_by(|a, b| {
        if result_to_value(&a.result) == result_to_value(&b.result) {
            let index = get_index_of_first_diff(&a.hand, &b.hand);
            let a1 = get_card_value(a.hand.get(index).unwrap());
            let b1 = get_card_value(b.hand.get(index).unwrap());
            b1.cmp(&a1)
        } else {
            result_to_value(&b.result).cmp(&result_to_value(&a.result))
        }
    });
    let mut total = 0;
    let len = plays.len();
    for i in 0..len {
        total += (len - i) * plays.get(i).unwrap().bid;
    }
    Some(total)
}

fn result_to_value(play_result: &PlayResult) -> u32 {
    match play_result {
        HighCard => 1,
        OnePair => 2,
        TwoPair => 3,
        ThreeOfAKind => 4,
        FullHouse => 5,
        FourOfAKind => 6,
        FiveOfAKind => 7,
    }
}

fn get_index_of_first_diff(first: &Vec<String>, second: &Vec<String>) -> usize {
    for i in 0..first.len() {
        let a = first.get(i).unwrap();
        let b = second.get(i).unwrap();
        if a != b {
            return i
        }
    }
    return 0
}

fn get_card_value(card: &String) -> usize {
    let parse = card.parse::<usize>();
    match parse {
        Ok(ok) => ok,
        Err(_) => {
            match card.as_str() {
                "T" => 10,
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => panic!("Invalid entry {}", card.as_str())
            }
        }
    }
}

fn get_result(hand: &Vec<String>) -> PlayResult {
    let mut occurrences: HashMap<String, usize> = HashMap::new();
    hand.iter().for_each(|h| {
        *occurrences.entry(h.clone()).or_insert(0) += 1;
    });
    let mut count_list: Vec<(&String, &usize)> = occurrences.iter().collect();
    count_list.sort_by(|a, b| b.1.cmp(a.1));
    let first = count_list.get(0).unwrap();
    let first_n = *first.1;
    if first_n == 5 {
        FiveOfAKind
    } else if first_n == 4 {
        FourOfAKind
    } else if first_n == 3 && *count_list.get(1).unwrap().1 == 2 {
        FullHouse
    } else if first_n == 3 {
        ThreeOfAKind
    } else if first_n == 2 && *count_list.get(1).unwrap().1 == 2 {
        TwoPair
    } else if first_n == 2 {
        OnePair
    } else {
        HighCard
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut plays: Vec<Play> = input.lines().map(|line| {
        let parts: Vec<String> = line.split(" ").map(|s| s.trim().to_string()).collect();
        let hand: Vec<String> = parts.get(0).unwrap().split("").map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        let result = get_result_prt2(&hand);
        Play {
            hand,
            bid: parts.get(1).unwrap().parse::<usize>().unwrap(),
            result,
        }
    }).collect();
    plays.sort_by(|a, b| {
        if result_to_value(&a.result) == result_to_value(&b.result) {
            let index = get_index_of_first_diff(&a.hand, &b.hand);
            let a1 = get_card_value_prt2(a.hand.get(index).unwrap());
            let b1 = get_card_value_prt2(b.hand.get(index).unwrap());
            b1.cmp(&a1)
        } else {
            result_to_value(&b.result).cmp(&result_to_value(&a.result))
        }
    });
    let mut total = 0;
    let len = plays.len();
    for i in 0..len {
        total += (len - i) * plays.get(i).unwrap().bid;
    }
    Some(total)
}

fn get_card_value_prt2(card: &String) -> usize {
    let parse = card.parse::<usize>();
    match parse {
        Ok(ok) => ok,
        Err(_) => {
            match card.as_str() {
                "J" => 1,
                "T" => 10,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => panic!("Invalid entry {}", card.as_str())
            }
        }
    }
}

fn get_result_prt2(hand: &Vec<String>) -> PlayResult {
    let mut occurrences: HashMap<String, usize> = HashMap::new();
    hand.iter().for_each(|h| {
        *occurrences.entry(h.clone()).or_insert(0) += 1;
    });
    let mut count_list: Vec<(&String, &usize)> = occurrences.iter().collect();
    count_list.sort_by(|a, b| b.1.cmp(a.1));
    let first = count_list.get(0).unwrap();
    let first_n = *first.1;
    if first_n == 5 {
        FiveOfAKind
    } else if first_n == 4 {
        let js = occurrences.get("J");
        match js {
            Some(4) => FiveOfAKind,
            Some(1) => FiveOfAKind,
            _ => FourOfAKind,
        }
    } else if first_n == 3 && *count_list.get(1).unwrap().1 == 2 {
        let js = occurrences.get("J");
        match js {
            Some(3) => FiveOfAKind,
            Some(2) => FiveOfAKind,
            _ => FullHouse,
        }
    } else if first_n == 3 {
        let js = occurrences.get("J");
        match js {
            Some(3) => FourOfAKind,
            Some(1) => FourOfAKind,
            _ => ThreeOfAKind,
        }
    } else if first_n == 2 && *count_list.get(1).unwrap().1 == 2 {
        if first.0 == &"J".to_string() {
            FourOfAKind
        } else {
            let js = occurrences.get("J");
            match js {
                Some(2) => FourOfAKind,
                Some(1) => FullHouse,
                _ => TwoPair,
            }
        }
    } else if first_n == 2 {
        if first.0 == &"J".to_string() {
            ThreeOfAKind
        } else {
            let js = occurrences.get("J");
            match js {
                Some(1) => ThreeOfAKind,
                _ => OnePair,
            }
        }
    } else {
        let js = occurrences.get("J");
        match js {
            Some(1) => OnePair,
            _ => HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
