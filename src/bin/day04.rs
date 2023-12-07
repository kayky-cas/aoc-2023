fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, cards) = line.split_once(':').unwrap();
            let (winnings, my_cards) = cards
                .split_once('|')
                .map(|(winnings, my_cards)| (winnings.trim(), my_cards.trim()))
                .map(|(winnings, my_cards)| {
                    let winnings: Vec<usize> =
                        winnings.split(' ').filter_map(|x| x.parse().ok()).collect();

                    let mut my_cards: Vec<usize> =
                        my_cards.split(' ').filter_map(|x| x.parse().ok()).collect();

                    my_cards.sort();

                    (winnings, my_cards)
                })
                .unwrap();

            let mut points = 0;

            for winning in winnings {
                if my_cards.binary_search(&winning).is_ok() {
                    if points == 0 {
                        points += 1;
                    } else {
                        points *= 2;
                    }
                }
            }

            points
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let cards: Vec<usize> = input
        .lines()
        .map(|line| {
            let (_, cards) = line.split_once(':').unwrap();
            let (winnings, my_cards) = cards
                .split_once('|')
                .map(|(winnings, my_cards)| (winnings.trim(), my_cards.trim()))
                .map(|(winnings, my_cards)| {
                    let winnings: Vec<usize> =
                        winnings.split(' ').filter_map(|x| x.parse().ok()).collect();

                    let mut my_cards: Vec<usize> =
                        my_cards.split(' ').filter_map(|x| x.parse().ok()).collect();

                    my_cards.sort();

                    (winnings, my_cards)
                })
                .unwrap();

            let mut points = 0;

            for winning in winnings {
                for my_card in &my_cards {
                    if *my_card > winning {
                        break;
                    }

                    if *my_card == winning {
                        points += 1;
                    }
                }
            }

            points
        })
        .collect();

    let mut instances = vec![1; cards.len()];

    for (idx, matched) in cards.iter().enumerate() {
        for i in idx + 1..=(idx + *matched).min(cards.len() - 1) {
            instances[i] += instances[idx];
        }
    }

    instances.iter().sum()
}

fn main() {
    let input = include_str!("../../input/day04.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30)
    }
}
