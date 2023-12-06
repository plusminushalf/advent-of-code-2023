use std::fs;

fn _iterate_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}

pub fn part1(_filename: &str) -> u32 {
    return 5;
}

pub fn part2(_filename: &str) -> u32 {
    return 5;
}
