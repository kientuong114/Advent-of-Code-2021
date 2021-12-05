use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

fn part1() -> usize {
    let re = Regex::new(r#"^(\d+),(\d+) -> (\d+),(\d+)$"#).unwrap();
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| re
             .captures(line)
             .unwrap()
        )
        .map(|c| Line {
            x1: c[1].parse().unwrap(),
            y1: c[2].parse().unwrap(),
            x2: c[3].parse().unwrap(),
            y2: c[4].parse().unwrap()
        });

    let mut count = HashMap::<(i32, i32), i32>::new();

    lines
        .filter(|line| line.x1 == line.x2 || line.y1 == line.y2)
        .for_each(|line| {
            let direction = if line.x1 == line.x2 {
                (0, (line.y2 - line.y1) / (line.y2 - line.y1).abs())
            } else {
                ((line.x2 - line.x1) / (line.x2 - line.x1).abs(), 0)
            };

            let mut curr = (line.x1, line.y1);

            loop {
                let n = count
                    .entry(curr)
                    .or_default();
                *n += 1;
                if curr == (line.x2, line.y2) {
                    break;
                }
                curr = (curr.0 + direction.0, curr.1 + direction.1)
            }
        });

    count
        .iter()
        .filter(|(_, v)| **v >= 2)
        .count()
}

fn part2() -> usize {
    let re = Regex::new(r#"^(\d+),(\d+) -> (\d+),(\d+)$"#).unwrap();
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| re
             .captures(line)
             .unwrap()
        )
        .map(|c| Line {
            x1: c[1].parse().unwrap(),
            y1: c[2].parse().unwrap(),
            x2: c[3].parse().unwrap(),
            y2: c[4].parse().unwrap()
        });

    let mut count = HashMap::<(i32, i32), i32>::new();

    lines
        .for_each(|line| {
            let direction = if line.x1 == line.x2 {
                (0, (line.y2 - line.y1) / (line.y2 - line.y1).abs())
            } else if line.y1 == line.y2 {
                ((line.x2 - line.x1) / (line.x2 - line.x1).abs(), 0)
            } else {
                ((line.x2 - line.x1) / (line.x2 - line.x1).abs(), (line.y2 - line.y1) / (line.y2 - line.y1).abs())
            };

            let mut curr = (line.x1, line.y1);

            loop {
                let n = count
                    .entry(curr)
                    .or_default();
                *n += 1;
                if curr == (line.x2, line.y2) {
                    break;
                }
                curr = (curr.0 + direction.0, curr.1 + direction.1)
            }
        });

    count
        .iter()
        .filter(|(_, v)| **v >= 2)
        .count()
}

fn main() {
    println!(
        "{}, {}",
        part1(),
        part2()
    )
}
