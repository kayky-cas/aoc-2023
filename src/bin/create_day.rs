use std::io::Write;

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap();

    let day = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} <day>", program_name);
        std::process::exit(1);
    });

    let day: u8 = day.parse().unwrap_or_else(|_| {
        eprintln!("Usage: {} <day>", program_name);
        std::process::exit(1);
    });

    let path = format!("src/bin/day{:02}.rs", day);

    let contents = format!(
        r#"fn part1(_input: &str) -> () {{
}}

fn part2(_input: &str) -> () {{
}}

fn main() {{
    let input = include_str!("../../input/day{day:02}.aoc");
    println!("Part 1: {{:?}}", part1(input));
    println!("Part 2: {{:?}}", part2(input));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_part1() {{
        let input = r"";
        assert_eq!(part1(input), ());
    }}

    #[test]
    fn test_part2() {{
        let input = r"";
        assert_eq!(part2(input), ());
    }}
}}
"#
    );

    let mut file = std::fs::File::create(&path)
        .map_err(|e| {
            eprintln!("Failed to create file: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    file.write_all(contents.as_bytes())
        .map_err(|e| {
            eprintln!("Failed to write to file: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    println!("Created {}", path);

    let path = format!("input/day{:02}.aoc", day);

    let _ = std::fs::File::create(&path)
        .map_err(|e| {
            eprintln!("Failed to create file: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    println!("Created {}", path);
}
