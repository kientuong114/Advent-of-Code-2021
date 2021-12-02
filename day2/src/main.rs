use regex::Regex;

fn part1() -> u32 {
    let re = Regex::new(r#"^([a-z]+) (\d+)$"#).unwrap();

    let (depth, pos) = include_str!("./input.txt")
        .lines()
        .map(|line| re.captures(line).unwrap())
        .fold((0u32, 0u32), |(depth, pos), line| {
            let x = line[2].parse::<u32>().unwrap();
            match &line[1] {
                "forward" => (depth, pos + x),
                "down" => (depth + x, pos),
                "up" => (depth - x, pos),
                _ => panic!("Got unexpected value: {:?}", line)
            }
        });

    depth * pos
}

fn part2() -> u32 {
    let re = Regex::new(r#"^([a-z]+) (\d+)$"#).unwrap();

    let (depth, pos, _) = include_str!("./input.txt")
        .lines()
        .map(|line| re.captures(line).unwrap())
        .fold((0u32, 0u32, 0u32), |(depth, pos, aim), line| {
            let x = line[2].parse::<u32>().unwrap();
            match &line[1] {
                "forward" => (depth + x * aim, pos + x, aim),
                "down" => (depth, pos, aim + x),
                "up" => (depth, pos, aim - x),
                _ => panic!("Got unexpected value: {:?}", line)
            }
        });

    depth * pos
}

fn main() {
    println!("{}, {}", part1(), part2());
}
