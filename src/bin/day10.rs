use std::collections::{HashSet, VecDeque};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Pipe {
    x: usize,
    y: usize,
    ch: char,
    directions: Vec<Direction>,
    distance: Option<usize>,
}

impl Pipe {
    fn new(ch: char, x: usize, y: usize, directions: Vec<Direction>) -> Self {
        Self {
            ch,
            x,
            y,
            directions,
            distance: None,
        }
    }

    fn from(c: char, x: usize, y: usize) -> Self {
        let directions = match c {
            '|' => vec![Direction::Up, Direction::Down],
            '-' => vec![Direction::Left, Direction::Right],
            'L' => vec![Direction::Right, Direction::Up],
            'J' => vec![Direction::Left, Direction::Up],
            '7' => vec![Direction::Left, Direction::Down],
            'F' => vec![Direction::Right, Direction::Down],
            _ => vec![],
        };

        Self::new(c, x, y, directions)
    }
}

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Pipe::from(c, x, y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .flat_map(|row| row.iter().filter(|c| c.ch == 'S').map(|p| (p.x, p.y)))
        .next()
        .unwrap();

    if start.1 > 0 {
        if grid[start.1 - 1][start.0]
            .directions
            .contains(&Direction::Down)
        {
            grid[start.1][start.0].directions.push(Direction::Up);
        }
    }

    if start.1 < grid.len() - 1 {
        if grid[start.1 + 1][start.0]
            .directions
            .contains(&Direction::Up)
        {
            grid[start.1][start.0].directions.push(Direction::Down);
        }
    }

    if start.0 > 0 {
        if grid[start.1][start.0 - 1]
            .directions
            .contains(&Direction::Right)
        {
            grid[start.1][start.0].directions.push(Direction::Left);
        }
    }

    if start.0 < grid[0].len() - 1 {
        if grid[start.1][start.0 + 1]
            .directions
            .contains(&Direction::Left)
        {
            grid[start.1][start.0].directions.push(Direction::Right);
        }
    }

    let mut queue: VecDeque<_> = vec![start].into();
    let mut max_distance = 0;

    while let Some(pipe) = queue.pop_front() {
        let distance = if let Some(distance) = grid[pipe.1][pipe.0].distance {
            if distance > max_distance {
                max_distance = distance;
            }
            distance + 1
        } else {
            1
        };

        for direction in grid[pipe.1][pipe.0].directions.clone() {
            match direction {
                Direction::Up => {
                    if pipe.1 > 0 {
                        let next_pipe = &grid[pipe.1 - 1][pipe.0];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Down)
                        {
                            grid[pipe.1 - 1][pipe.0].distance = Some(distance);
                            queue.push_back((pipe.0, pipe.1 - 1));
                        }
                    }
                }
                Direction::Down => {
                    if pipe.1 < grid.len() - 1 {
                        let next_pipe = &grid[pipe.1 + 1][pipe.0];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Up)
                        {
                            grid[pipe.1 + 1][pipe.0].distance = Some(distance);
                            queue.push_back((pipe.0, pipe.1 + 1));
                        }
                    }
                }
                Direction::Left => {
                    if pipe.0 > 0 {
                        let next_pipe = &grid[pipe.1][pipe.0 - 1];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Right)
                        {
                            grid[pipe.1][pipe.0 - 1].distance = Some(distance);
                            queue.push_back((pipe.0 - 1, pipe.1));
                        }
                    }
                }
                Direction::Right => {
                    if pipe.0 < grid[0].len() - 1 {
                        let next_pipe = &grid[pipe.1][pipe.0 + 1];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Left)
                        {
                            grid[pipe.1][pipe.0 + 1].distance = Some(distance);
                            queue.push_back((pipe.0 + 1, pipe.1));
                        }
                    }
                }
            }
        }
    }

    max_distance
}

fn part2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Pipe::from(c, x, y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .flat_map(|row| row.iter().filter(|c| c.ch == 'S').map(|p| (p.x, p.y)))
        .next()
        .unwrap();

    if start.1 > 0 {
        if grid[start.1 - 1][start.0]
            .directions
            .contains(&Direction::Down)
        {
            grid[start.1][start.0].directions.push(Direction::Up);
        }
    }

    if start.1 < grid.len() - 1 {
        if grid[start.1 + 1][start.0]
            .directions
            .contains(&Direction::Up)
        {
            grid[start.1][start.0].directions.push(Direction::Down);
        }
    }

    if start.0 > 0 {
        if grid[start.1][start.0 - 1]
            .directions
            .contains(&Direction::Right)
        {
            grid[start.1][start.0].directions.push(Direction::Left);
        }
    }

    if start.0 < grid[0].len() - 1 {
        if grid[start.1][start.0 + 1]
            .directions
            .contains(&Direction::Left)
        {
            grid[start.1][start.0].directions.push(Direction::Right);
        }
    }

    let mut pipe_loop = HashSet::new();
    let mut queue: VecDeque<_> = vec![start].into();
    while let Some(pipe) = queue.pop_front() {
        if pipe_loop.contains(&pipe) {
            continue;
        }

        pipe_loop.insert(pipe);

        let distance = if let Some(distance) = grid[pipe.1][pipe.0].distance {
            distance + 1
        } else {
            1
        };

        for direction in grid[pipe.1][pipe.0].directions.clone() {
            match direction {
                Direction::Up => {
                    if pipe.1 > 0 {
                        let next_pipe = &grid[pipe.1 - 1][pipe.0];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Down)
                        {
                            grid[pipe.1 - 1][pipe.0].distance = Some(distance);
                            queue.push_back((pipe.0, pipe.1 - 1));
                        }
                    }
                }
                Direction::Down => {
                    if pipe.1 < grid.len() - 1 {
                        let next_pipe = &grid[pipe.1 + 1][pipe.0];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Up)
                        {
                            grid[pipe.1 + 1][pipe.0].distance = Some(distance);
                            queue.push_back((pipe.0, pipe.1 + 1));
                        }
                    }
                }
                Direction::Left => {
                    if pipe.0 > 0 {
                        let next_pipe = &grid[pipe.1][pipe.0 - 1];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Right)
                        {
                            grid[pipe.1][pipe.0 - 1].distance = Some(distance);
                            queue.push_back((pipe.0 - 1, pipe.1));
                        }
                    }
                }
                Direction::Right => {
                    if pipe.0 < grid[0].len() - 1 {
                        let next_pipe = &grid[pipe.1][pipe.0 + 1];
                        if next_pipe.ch != 'S'
                            && next_pipe.distance.is_none()
                            && next_pipe.directions.contains(&Direction::Left)
                        {
                            grid[pipe.1][pipe.0 + 1].distance = Some(distance);
                            queue.push_back((pipe.0 + 1, pipe.1));
                        }
                    }
                }
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|pipe| {
            !pipe_loop.contains(&(pipe.x, pipe.y))
                && (0..pipe.x)
                    .filter(|&x| {
                        pipe_loop.contains(&(x, pipe.y))
                            && grid[pipe.y][x].directions.contains(&Direction::Up)
                    })
                    .count()
                    % 2
                    == 1
        })
        .count()
}

fn main() {
    let input = include_str!("../../input/day10.aoc");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part2() {
        let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(input), 4);

        let input = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(part2(input), 4);

        let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(input), 8);

        let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input), 10);
    }
}
