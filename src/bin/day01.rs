fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

            let mut number = String::from(first);
            number.push(last);

            number.parse::<usize>().unwrap()
        })
        .sum()
}

fn part2_handle(sl: &str, table: &[&str]) -> Option<char> {
    match sl.len() {
        6.. => None,
        1 => {
            let ch = sl.chars().next().unwrap();
            if ch.is_digit(10) {
                Some(ch)
            } else {
                None
            }
        }
        _ => {
            if let Some(pos) = table.iter().position(|&x| x == sl) {
                Some(char::from_digit((pos + 1) as u32, 10).unwrap())
            } else {
                None
            }
        }
    }
}

fn part2(input: &str) -> usize {
    let table = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let mut first = '\0';

            'out: for i in 0..line.len() {
                for j in i..line.len() {
                    let sl = &line[i..=j];

                    if let Some(ch) = part2_handle(sl, &table) {
                        first = ch;
                        break 'out;
                    }
                }
            }

            let mut last = '\0';

            'out: for i in (0..line.len()).rev() {
                for j in (0..=i).rev() {
                    let sl = &line[j..=i];

                    if let Some(ch) = part2_handle(sl, &table) {
                        last = ch;
                        break 'out;
                    }
                }
            }

            let mut number = String::from(first);
            number.push(last);

            number.parse::<usize>().unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/day01.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
