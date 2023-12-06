use num_traits::pow;
use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let total: usize = input
        .lines()
        .map(|line| {
            let split: Vec<String> = line.split("|").map(|s| s.trim().to_string()).collect();
            let winning: Vec<u32> = split[0]
                .clone()
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect();
            let hand: Vec<u32> = split[1]
                .clone()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect();
            let winning_numbers: Vec<&u32> = winning.iter().filter(|w| hand.contains(w)).collect();
            if winning_numbers.is_empty() {
                0
            } else {
                let i = winning_numbers.len() - 1;
                let a = pow(2, i);
                a
            }
        })
        .sum();
    Some(total)
}

struct CardAndHand {
    card_number: u32,
    winning: Vec<u32>,
    hand: Vec<u32>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let hand_by_card_number: HashMap<u32, CardAndHand> = input
        .lines()
        .map(|line| {
            let split: Vec<String> = line.split("|").map(|s| s.trim().to_string()).collect();
            let cards_parts: Vec<String> = split[0]
                .clone()
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let card_number: u32 = cards_parts[0]
                .clone()
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .nth(1)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();
            let winning: Vec<u32> = cards_parts
                .get(1)
                .unwrap()
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect();
            let hand: Vec<u32> = split[1]
                .clone()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect();
            let card_and_hand = CardAndHand {
                card_number,
                winning,
                hand,
            };
            (card_number, card_and_hand)
        })
        .collect();

    let mut number_of_copies: HashMap<u32, u32> = (1..hand_by_card_number.len() + 1)
        .map(|i| (i as u32, 1))
        .collect();

    for i in 1..hand_by_card_number.len() + 1 {
        let card_and_hand = hand_by_card_number.get(&(i as u32)).unwrap();
        let winning: Vec<u32> = card_and_hand.winning.clone();
        let card_number: u32 = card_and_hand.card_number;
        let hand: Vec<u32> = card_and_hand.hand.clone();
        let winning_hand: Vec<&u32> = winning.iter().filter(|w| hand.contains(w)).collect();
        if !winning_hand.is_empty() {
            let number_of_current_number_copies = *number_of_copies.entry(card_number).or_default();
            for i in 1..winning_hand.len() + 1 {
                *number_of_copies.entry(card_number + i as u32).or_default() +=
                    number_of_current_number_copies;
            }
        }
    }
    let total = number_of_copies.values().sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
