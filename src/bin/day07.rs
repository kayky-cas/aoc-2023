use std::{cmp::Ordering, collections::HashMap};

struct Game {
    hand: Vec<char>,
    bid: usize,
    stats: Stats,
}

struct Stats {
    max: usize,
    len: usize,
}

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

            let map = hand.chars().fold(HashMap::new(), |mut acc, card| {
                if let Some(count) = acc.get(&card) {
                    acc.insert(card, count + 1);
                } else {
                    acc.insert(card, 1);
                }

                acc
            });

            let max = map.values().max().unwrap_or(&0);
            let len = map.len();

            let stats = Stats { max: *max, len };

            Game {
                hand: cards,
                bid,
                stats,
            }
        })
        .collect();

    game.sort_by(|a, b| {
        match a.stats.max.cmp(&b.stats.max) {
            Ordering::Equal => {}
            ord => return ord,
        };

        match b.stats.len.cmp(&a.stats.len) {
            Ordering::Equal => {}
            ord => return ord,
        };

        for (card_a, card_b) in a.hand.iter().zip(b.hand.iter()) {
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
        .map(|(idx, game)| (idx + 1) * game.bid)
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

            let map = hand.chars().fold(HashMap::new(), |mut acc, card| {
                if let Some(count) = acc.get(&card) {
                    acc.insert(card, count + 1);
                } else {
                    acc.insert(card, 1);
                }

                acc
            });

            let max: usize = map
                .iter()
                .filter(|(&card, _)| card != 'J')
                .map(|(_, c)| c)
                .max()
                .unwrap_or(&0)
                + *map.get(&'J').unwrap_or(&0);

            let len = map.iter().filter(|(&card, _)| card != 'J').count().max(1);

            let stats = Stats { max, len };

            Game {
                hand: cards,
                bid,
                stats,
            }
        })
        .collect();

    game.sort_by(|a, b| {
        match a.stats.max.cmp(&b.stats.max) {
            Ordering::Equal => {}
            ord => return ord,
        };

        match b.stats.len.cmp(&a.stats.len) {
            Ordering::Equal => {}
            ord => return ord,
        };

        for (card_a, card_b) in a.hand.iter().zip(b.hand.iter()) {
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
        .map(|(idx, game)| (idx + 1) * game.bid)
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
