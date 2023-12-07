fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    lines
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .zip(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .flat_map(|x| x.parse::<usize>()),
        )
        .map(|(time, distance)| {
            (1..time)
                .map(|i| {
                    let time_left = time - i;
                    time_left * i
                })
                .filter(|x| x > &distance)
                .count()
        })
        .product()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .fold(String::new(), |acc, x| format!("{}{}", acc, x))
        .parse()
        .unwrap();

    let distance: u128 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .fold(String::new(), |acc, x| format!("{}{}", acc, x))
        .parse()
        .unwrap();

    (1..time)
        .map(|i| {
            let time_left = time - i;
            time_left * i
        })
        .filter(|x| x > &distance)
        .count()
}

fn main() {
    let input = include_str!("../../input/day06.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part2(input), 71503);
    }
}
