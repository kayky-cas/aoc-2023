use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers: Vec<(String, (usize, usize))> = Vec::new();
    let mut count = 0;

    for i in 0..schematic.len() {
        for j in 0..schematic[0].len() {
            let ch = schematic[i][j];

            if ch.is_ascii_digit() {
                if count == numbers.len() {
                    numbers.push((String::from(ch), (j, i)));
                } else {
                    numbers[count].0.push(ch);
                }
            } else if count != numbers.len() {
                count += 1;
            }
        }
    }

    numbers
        .iter()
        .filter_map(|num| {
            for i in num.1 .1.checked_sub(1).unwrap_or(num.1 .1)
                ..=(num.1 .1 + 1).min(schematic.len() - 1)
            {
                for j in num.1 .0.checked_sub(1).unwrap_or(num.1 .0)
                    ..=(num.1 .0 + num.0.len()).min(schematic.len() - 1)
                {
                    if i == num.1 .1 && j >= num.1 .0 && j < num.1 .0 + num.0.len() {
                        continue;
                    }
                    let ch = schematic[i][j];
                    if !ch.is_ascii_digit() && ch != '.' {
                        return num.0.parse().ok();
                    }
                }
            }

            None::<usize>
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers: Vec<(String, (usize, usize))> = Vec::new();
    let mut count = 0;

    let mut mult: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for i in 0..schematic.len() {
        for j in 0..schematic[0].len() {
            let ch = schematic[i][j];

            if ch.is_ascii_digit() {
                if count == numbers.len() {
                    numbers.push((String::from(ch), (j, i)));
                } else {
                    numbers[count].0.push(ch);
                }
            } else {
                if ch == '*' {
                    mult.insert((j, i), vec![]);
                }

                if count != numbers.len() {
                    count += 1;
                }
            }
        }
    }

    for num in numbers {
        for i in
            num.1 .1.checked_sub(1).unwrap_or(num.1 .1)..=(num.1 .1 + 1).min(schematic.len() - 1)
        {
            for j in num.1 .0.checked_sub(1).unwrap_or(num.1 .0)
                ..=(num.1 .0 + num.0.len()).min(schematic.len() - 1)
            {
                if i == num.1 .1 && j >= num.1 .0 && j < num.1 .0 + num.0.len() {
                    continue;
                }
                let ch = schematic[i][j];
                if !ch.is_ascii_digit() && ch != '.' && ch == '*' {
                    mult.get_mut(&(j, i)).unwrap().push(num.0.parse().unwrap())
                }
            }
        }
    }

    mult.iter()
        .filter_map(|(_, value)| {
            if value.len() == 1 {
                None
            } else {
                Some(value.iter().product::<usize>())
            }
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../../input/day03.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(input), 467835);
    }
}
