fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut current: Vec<isize> = line
                .trim()
                .split_whitespace()
                .flat_map(|num| num.parse())
                .collect();

            let mut diffs = vec![];

            while !current.iter().all(|&x| x == 0) {
                diffs.push(*current.last().unwrap());
                current = current.windows(2).map(|w| w[1] - w[0]).collect();
            }

            diffs.iter().rev().fold(0, |acc, x| acc + x)
        })
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut current: Vec<isize> = line
                .trim()
                .split_whitespace()
                .flat_map(|num| num.parse())
                .collect();

            let mut diffs = vec![];

            while !current.iter().all(|&x| x == 0) {
                diffs.push(*current.first().unwrap());
                current = current.windows(2).map(|w| w[1] - w[0]).collect();
            }

            diffs.iter().rev().fold(0, |acc, x| x - acc)
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/day09.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(input), 114)
    }

    #[test]
    fn test_part2() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part2(input), 2);
    }
}
