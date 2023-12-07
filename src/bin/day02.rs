fn part1(input: &str) -> usize {
    let bag = [12, 13, 14];

    input
        .lines()
        .enumerate()
        .filter(|(_, s)| {
            let (_, content) = s.trim().split_once(':').unwrap();

            for play in content.split(',').flat_map(|s| s.split(';')) {
                let (quant, color) = play.trim_start().split_once(' ').unwrap();

                let color = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };

                if quant.parse::<usize>().unwrap() > bag[color] {
                    return false;
                }
            }

            true
        })
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let (_, content) = s.trim().split_once(':').unwrap();

            let mut bag = [0, 0, 0];

            for play in content.split(',').flat_map(|s| s.split(';')) {
                let (quant, color) = play.trim_start().split_once(' ').unwrap();

                let color = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };

                let quant = quant.parse().unwrap();

                if quant > bag[color] {
                    bag[color] = quant;
                }
            }

            bag[0] * bag[1] * bag[2]
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/day02.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286);
    }
}
