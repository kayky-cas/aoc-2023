fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut offset = 0;

    for j in 0..grid.len() {
        if grid[j + offset].iter().all(|&c| c == '.') {
            grid.insert(j + offset, vec!['.'; grid[j + offset].len()]);
            offset += 1;
        }
    }

    let mut offset = 0;

    for i in 0..grid[0].len() {
        if grid.iter().map(|g| g[i + offset]).all(|c| c == '.') {
            for j in 0..grid.len() {
                grid[j].insert(i + offset, '.');
            }

            offset += 1;
        }
    }

    let mut galaxies = vec![];

    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            if grid[j][i] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let distance = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs()
                + (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs();

            sum += distance as usize;
        }
    }

    sum
}

fn part2(input: &str, expends: usize) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 1)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for j in 0..grid.len() {
        if grid[j].iter().all(|&(c, _)| c == '.') {
            for i in 0..grid[j].len() {
                grid[j][i] = ('.', grid[j][i].1 * expends);
            }
        }
    }

    for i in 0..grid[0].len() {
        if grid.iter().map(|g| g[i]).all(|(c, _)| c == '.') {
            for j in 0..grid.len() {
                grid[j][i] = ('.', grid[j][i].1 * expends);
            }
        }
    }

    let mut galaxies = vec![];

    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            if grid[j][i].0 == '#' {
                let mut offset_j = 0;
                for k in 0..j {
                    offset_j += grid[k][i].1;
                }

                let mut offset_i = 0;

                for k in 0..i {
                    offset_i += grid[j][k].1;
                }

                galaxies.push((offset_i, offset_j));
            }
        }
    }

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let distance = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs()
                + (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs();

            sum += distance as usize;
        }
    }

    sum
}

fn main() {
    let input = include_str!("../../input/day11.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn test_part2() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part2(input, 10), 1030);
        assert_eq!(part2(input, 100), 8410);
    }
}
