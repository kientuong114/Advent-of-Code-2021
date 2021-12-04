use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;

fn split_line(line: &&str) -> Vec<i32> {
    let re = Regex::new(r#"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)$"#).unwrap();

    re
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|n| n
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap()
        )
        .collect_vec()
}

fn check_single_table(table: &Vec<Vec<i32>>, set: &HashSet<i32>) -> bool {
    table
        .iter()
        .any(|row| row
             .iter()
             .all(|&n| set.contains(&n))
        )
}

fn check_table(table: &Vec<Vec<i32>>, set: &HashSet<i32>) -> Option<i32> {
    let table_transpose = (0..table[0].len())
        .map(|i| table
             .iter()
             .map(|c| c[i])
             .collect_vec()
        ).collect_vec();

    if check_single_table(table, set) || check_single_table(&table_transpose, set) {
        let res: i32 = table
            .iter()
            .map::<i32, _>(|row: &Vec<i32>| row
                 .iter()
                 .filter(|n| !set.contains(n))
                 .sum()
            ).sum();
        Some(res)
    } else {
        None
    }
}

fn part1() -> i32 {
    let input = include_str!("./input.txt")
        .lines()
        .collect_vec();

    let called = input[0].split(",").map(|n| n.parse::<i32>().unwrap());

    let mut tables = Vec::new();
    let mut idx = 2;

    for i in 2..input.len() {
        if input[i] == "" {
            tables.push(
                input[idx..i]
                    .iter()
                    .map(split_line)
                    .collect_vec()
            );
            idx = i+1;
        }
    }

    tables.push(
        input[idx..]
            .iter()
            .map(split_line)
            .collect_vec()
    );

    let mut set = HashSet::<i32>::new();

    for c in called {
        set.insert(c);

        for table in tables.iter() {
            match check_table(table, &set) {
                Some(s) => {
                    return s * c;
                }
                None => {
                    continue;
                }
            }
        }
    }

    -1
}

fn part2() -> i32 {
    let input = include_str!("./input.txt")
        .lines()
        .collect_vec();

    let called = input[0].split(",").map(|n| n.parse::<i32>().unwrap());

    let mut tables = Vec::new();
    let mut idx = 2;

    for i in 2..input.len() {
        if input[i] == "" {
            tables.push(
                input[idx..i]
                    .iter()
                    .map(split_line)
                    .collect_vec()
            );
            idx = i+1;
        }
    }

    tables.push(
        input[idx..]
            .iter()
            .map(split_line)
            .collect_vec()
    );

    let mut set = HashSet::<i32>::new();

    for c in called {
        set.insert(c);

        if tables.len() == 1 {
            match check_table(&tables[0], &set) {
                Some(value) => {
                    return value * c;
                }
                None => {
                    continue;
                }
            }
        } else {
            tables.retain(|table| check_table(table, &set).is_none());
        }
    }

    -1
}

fn main() {
    print!("{} {}", part1(), part2())
}
