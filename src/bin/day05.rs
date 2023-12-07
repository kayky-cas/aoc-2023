use rayon::prelude::*;

#[derive(Debug)]
struct Instruction {
    start: usize,
    length: usize,
    offset: usize,
}

fn part1(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap());

    let maps: Vec<Vec<_>> = sections
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let (offset, start_length) = line.trim().split_once(' ').unwrap();
                    let (start, lenght) = start_length.split_once(' ').unwrap();

                    let start = start.parse::<usize>().unwrap();
                    let length = lenght.parse::<usize>().unwrap();
                    let offset = offset.parse::<usize>().unwrap();

                    Instruction {
                        start,
                        length,
                        offset,
                    }
                })
                .collect()
        })
        .collect();

    seeds
        .map(|seed| {
            let mut current = seed;
            for map in maps.iter() {
                for instruction in map {
                    if current >= instruction.start
                        && current < instruction.start + instruction.length
                    {
                        current = instruction.offset + (current - instruction.start);
                        break;
                    }
                }
            }
            current
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let seeds: Vec<usize> = sections
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let maps: Vec<Vec<_>> = sections
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let (offset, start_length) = line.trim().split_once(' ').unwrap();
                    let (start, lenght) = start_length.split_once(' ').unwrap();

                    let start = start.parse::<usize>().unwrap();
                    let length = lenght.parse::<usize>().unwrap();
                    let offset = offset.parse::<usize>().unwrap();

                    Instruction {
                        start,
                        length,
                        offset,
                    }
                })
                .collect()
        })
        .collect();

    seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .par_bridge()
        .map(|seed| {
            maps.iter().fold(seed, |acc, map| {
                for instruction in map {
                    if acc >= instruction.start && acc < instruction.start + instruction.length {
                        return instruction.offset + (acc - instruction.start);
                    }
                }
                acc
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/day05.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part2(input), 46);
    }
}
