use std::{collections::HashMap, fs};

fn iterate_file(filename: &str) -> Vec<Vec<String>> {
    return fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split("|")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
}

pub fn part1(filename: &str) -> u32 {
    iterate_file(filename)
        .iter()
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

            return line[1]
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .map(|number| *winning_numbers.get(&number).unwrap_or(&0))
                .filter(|number| *number != 0)
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });
        })
        .sum()
}

pub fn part2(filename: &str) -> u32 {
    let mut total_count: u32 = 0;
    let mut game_count_map: HashMap<usize, u32> = HashMap::new();

    iterate_file(filename)
        .iter()
        .enumerate()
        .for_each(|(index, game)| {
            let winning_numbers: HashMap<u32, u32> = game[0]
                .split_whitespace()
                .map(|number| {
                    (
                        number.parse::<u32>().unwrap(),
                        number.parse::<u32>().unwrap(),
                    )
                })
                .collect();

            let winnings: u32 = u32::try_from(
                game[1]
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .map(|number| *winning_numbers.get(&number).unwrap_or(&0))
                    .filter(|number| *number != 0)
                    .count(),
            )
            .unwrap_or(0);

            let multiplier = *game_count_map.get(&(index + 1)).unwrap_or(&1);

            for number in 1..(winnings + 1) {
                let game_index: usize = index + 1 + usize::try_from(number).unwrap();
                let count = game_count_map.entry(game_index).or_insert(1);
                *count += multiplier;
            }

            total_count += multiplier;
        });

    return total_count;
}
