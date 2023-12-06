use std::{collections::HashMap, fs};

fn iterate_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}

pub fn part1(filename: &str) -> u32 {
    iterate_file(filename)
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split("|")
                .collect::<Vec<&str>>()
        })
        .map(|line| {
            let winning_numbers: HashMap<u32, u32> = line[0]
                .split_whitespace()
                .map(|number| {
                    (
                        number.parse::<u32>().unwrap(),
                        number.parse::<u32>().unwrap(),
                    )
                })
                .collect();

            // 0 -> 1
            // 1 -> 2
            // 2 -> 4
            // 3 -> 8

            let score = line[1]
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .map(|number| *winning_numbers.get(&number).unwrap_or(&0))
                .filter(|number| *number != 0)
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });

            score
        })
        .sum()
}

pub fn part2(_filename: &str) -> u32 {
    return 5;
}
