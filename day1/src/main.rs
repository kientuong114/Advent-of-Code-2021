use itertools::Itertools;

fn part1() -> u32 {
    include_str!("./input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .fold((0u32, u32::MAX), |(count, prev), val| {
            if prev < val {
                (count + 1, val)
            } else {
                (count, val)
            }
        }).0
}

fn part2() -> u32 {
    include_str!("./input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows::<(_,_,_)>()
        .map(|t| t.0 + t.1 + t.2)
        .fold((0u32, u32::MAX), |(count, prev), val| {
            if prev < val {
                (count + 1, val)
            } else {
                (count, val)
            }
        }).0
}

fn main() {
    println!(
        "{}, {}",
        part1(),
        part2()
    )
}
