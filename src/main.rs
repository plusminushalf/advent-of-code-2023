use std::env;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let run_mode = if arguments.iter().any(|arg| arg == "input") {
        "input"
    } else {
        "test"
    };

    let functions = [
        day1::part1,
        day1::part2,
        day2::part1,
        day2::part2,
        day3::part1,
        day3::part2,
        day4::part1,
        day4::part2,
    ];

    let days = 5;

    println!();

    let mut function_index: usize = 0;

    for day in 1..days {
        print!("Day {}:", day);
        let filename: String = format!("src/day{}/{}", day, run_mode);
        println!();
        for part in 1..3 {
            print!("\tpart {}: {}", part, functions[function_index](&filename));
            function_index += 1;
            println!();
        }
    }
}
