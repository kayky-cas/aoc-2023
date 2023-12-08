use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();

    let map = lines
        .skip(1)
        .map(|line| {
            let (node, paths) = line.split_once(" = ").unwrap();
            let paths = paths.trim_matches(|c| c == '(' || c == ')');

            let paths = paths.split_once(", ").unwrap();

            (node, paths)
        })
        .fold(HashMap::new(), |mut map, (node, paths)| {
            map.insert(node, paths);
            map
        });

    let (mut left, mut right) = map.get("AAA").unwrap();

    instructions
        .take_while(|instruction| {
            let side = match instruction {
                'L' => left,
                'R' => right,
                _ => unreachable!(),
            };

            if side == "ZZZ" {
                false
            } else {
                let (l, r) = map.get(side).unwrap();

                left = l;
                right = r;

                true
            }
        })
        .count()
        + 1
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions: Vec<_> = lines.next().unwrap().chars().collect();

    let map = lines
        .skip(1)
        .map(|line| {
            let (node, paths) = line.split_once(" = ").unwrap();
            let paths = paths.trim_matches(|c| c == '(' || c == ')');

            let paths = paths.split_once(", ").unwrap();

            (node, paths)
        })
        .fold(HashMap::new(), |mut map, (node, paths)| {
            map.insert(node, paths);
            map
        });

    let paths: Vec<_> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| {
            let mut node = k;

            instructions
                .iter()
                .cycle()
                .take_while(|instruction| {
                    let (left, right) = map.get(node).unwrap();

                    let side = match instruction {
                        'L' => left,
                        'R' => right,
                        _ => unreachable!(),
                    };

                    node = side;

                    !side.ends_with("Z")
                })
                .count()
                + 1
        })
        .collect();

    min_mult_com(&paths)
}

fn min_mult_com(nums: &[usize]) -> usize {
    match nums.len() {
        0 => 0,
        1 => nums[0],
        _ => {
            let a = nums[0];
            let b = min_mult_com(&nums[1..]);

            a * b / max_div_com(a, b)
        }
    }
}

fn max_div_com(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    max_div_com(b, a % b)
}

fn main() {
    let input = include_str!("../../input/day08.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 2);

        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
