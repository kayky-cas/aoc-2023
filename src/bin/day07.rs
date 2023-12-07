use std::{cmp::Ordering, collections::HashMap};

fn part1(input: &str) -> usize {
    let points = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let mut game: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            let bid: usize = bid.parse().unwrap();
            let cards: Vec<char> = hand.chars().collect();
            (cards, bid)
        })
        .collect();

    game.sort_by(|a, b| {
        let a_map = a.0.iter().copied().fold(HashMap::new(), |mut acc, card| {
            if let Some(count) = acc.get(&card) {
                acc.insert(card, count + 1);
            } else {
                acc.insert(card, 1);
            }

            acc
        });

        let b_map = b.0.iter().copied().fold(HashMap::new(), |mut acc, card| {
            if let Some(count) = acc.get(&card) {
                acc.insert(card, count + 1);
            } else {
                acc.insert(card, 1);
            }

            acc
        });

        let max_a = a_map.values().max();
        let max_b = b_map.values().max();

        match max_a.cmp(&max_b) {
            Ordering::Equal => {}
            ord => return ord,
        };

        match b_map.len().cmp(&a_map.len()) {
            Ordering::Equal => {}
            ord => return ord,
        };

        for (card_a, card_b) in a.0.iter().zip(b.0.iter()) {
            let a = points.iter().position(|card| *card == *card_a);
            let b = points.iter().position(|card| *card == *card_b);

            match b.cmp(&a) {
                Ordering::Equal => {}
                ord => return ord,
            };
        }

        Ordering::Equal
    });

    game.iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum()
}

fn part2(input: &str) -> usize {
    let points = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let mut game: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            let bid: usize = bid.parse().unwrap();
            let cards: Vec<char> = hand.chars().collect();
            (cards, bid)
        })
        .collect();

    game.sort_by(|a, b| {
        let a_map = a.0.iter().copied().fold(HashMap::new(), |mut acc, card| {
            if let Some(count) = acc.get(&card) {
                acc.insert(card, count + 1);
            } else {
                acc.insert(card, 1);
            }

            acc
        });

        let b_map = b.0.iter().copied().fold(HashMap::new(), |mut acc, card| {
            if let Some(count) = acc.get(&card) {
                acc.insert(card, count + 1);
            } else {
                acc.insert(card, 1);
            }

            acc
        });

        let max_a: usize = a_map
            .iter()
            .filter(|(&card, _)| card != 'J')
            .map(|(_, c)| c)
            .max()
            .unwrap_or(&0)
            + *a_map.get(&'J').unwrap_or(&0);

        let max_b: usize = b_map
            .iter()
            .filter(|(&card, _)| card != 'J')
            .map(|(_, c)| c)
            .max()
            .unwrap_or(&0)
            + *b_map.get(&'J').unwrap_or(&0);

        match max_a.cmp(&max_b) {
            Ordering::Equal => {}
            ord => return ord,
        };

        let a_len = a_map.iter().filter(|(&card, _)| card != 'J').count().max(1);
        let b_len = b_map.iter().filter(|(&card, _)| card != 'J').count().max(1);

        match b_len.cmp(&a_len) {
            Ordering::Equal => {}
            ord => return ord,
        };

        for (card_a, card_b) in a.0.iter().zip(b.0.iter()) {
            let a = points.iter().position(|card| *card == *card_a);
            let b = points.iter().position(|card| *card == *card_b);

            match b.cmp(&a) {
                Ordering::Equal => {}
                ord => return ord,
            };
        }

        Ordering::Equal
    });

    game.iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum()
}

fn main() {
    let input = include_str!("../../input/day07.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part2(input), 5905);
    }
}
