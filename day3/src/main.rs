use core::panic;
use std::convert::TryInto;

use itertools::Itertools;

fn part1() -> u32 {
    let lines = include_str!("./input.txt")
        .lines()
        .collect_vec();

    let mut count = vec![0i32; lines[0].len()];

    for line in &lines {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '0' => count[i] -= 1,
                '1' => count[i] += 1,
                _ => panic!("Got unexpected char {}", ch)
            }
        }
    }

    let gamma: u32 = count
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            if *c > 0 {
                acc + u32::pow(2, i.try_into().unwrap())
            } else {
                acc
            }
        });

    // Invert gamma bitwise to get epsilon
    let epsilon = u32::pow(2, lines[0].len().try_into().unwrap()) - 1 - gamma;

    gamma * epsilon
}

fn part2() -> u32 {
    let lines = include_str!("./input.txt")
        .lines()
        .collect_vec();

    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    for i in 0..lines[0].len() {
        // Determine majority

        let count_oxygen = oxygen
            .iter()
            .fold(0i32, |c, line| {
                match line.chars().nth(i).unwrap() {
                    '0' => c - 1,
                    '1' => c + 1,
                    _ => panic!("Got unexpected char")
                }
            });
        
        let count_co2 = co2
            .iter()
            .fold(0i32, |c, line| {
                match line.chars().nth(i).unwrap() {
                    '0' => c - 1,
                    '1' => c + 1,
                    _ => panic!("Got unexpected char")
                }
            });

        if oxygen.len() > 1 {
            oxygen.retain(|line| {
                let ch = line.chars().nth(i).unwrap();
                if count_oxygen >= 0 {
                    ch == '1'
                } else {
                    ch == '0'
                }
            });
        }

        if co2.len() > 1 {
            co2.retain(|line| {
                let ch = line.chars().nth(i).unwrap();
                if count_co2 >= 0 {
                    ch == '0'
                } else {
                    ch == '1'
                }
            });
        }
    }

    let oxygen_rating = u32::from_str_radix(oxygen[0], 2).unwrap();
    let co2_rating = u32::from_str_radix(co2[0], 2).unwrap();

    oxygen_rating * co2_rating
}

fn main() {
    println!("{}, {}", part1(), part2());
}
