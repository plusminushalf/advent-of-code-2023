use std::fs;

fn find_number(input: &Vec<Vec<char>>, i: usize, mut j: usize) -> u32 {
    while j >= 1 && input[i][j - 1].is_digit(10) {
        j -= 1;
    }

    let mut number: u32 = 0;

    while j < input[0].len() && input[i][j].is_digit(10) {
        number = number * 10 + input[i][j].to_digit(10).unwrap();
        j += 1;
    }

    return number;
}

fn near_symbol(input: &Vec<Vec<char>>, i: usize, j: usize) -> Result<u32, &'static str> {
    let mut top_left: u32 = 1;
    let mut top: u32 = 1;
    let mut top_right: u32 = 1;
    let mut left: u32 = 1;
    let mut right: u32 = 1;
    let mut bottom_left: u32 = 1;
    let mut bottom: u32 = 1;
    let mut bottom_right: u32 = 1;
    let mut found: bool = false;

    if i >= 1 && j >= 1 && input[i - 1][j - 1].is_digit(10) {
        top_left = find_number(input, i - 1, j - 1);
        found = true;
    }

    if i >= 1 && input[i - 1][j].is_digit(10) {
        top = find_number(input, i - 1, j);
        found = true;
    }

    if top_left == top {
        top_left = 1;
    }

    if i >= 1 && j + 1 < input[0].len() && input[i - 1][j + 1].is_digit(10) {
        top_right = find_number(input, i - 1, j + 1);
        found = true;
    }

    if top_left == top_right {
        top_left = 1;
    }

    if top == top_right {
        top = 1;
    }

    if j >= 1 && input[i][j - 1].is_digit(10) {
        left = find_number(input, i, j - 1);
        found = true;
    }

    if j + 1 < input[0].len() && input[i][j + 1].is_digit(10) {
        right = find_number(input, i, j + 1);
        found = true;
    }

    if i + 1 < input.len() && j >= 1 && input[i + 1][j - 1].is_digit(10) {
        bottom_left = find_number(input, i + 1, j - 1);
        found = true;
    }

    if i + 1 < input.len() && input[i + 1][j].is_digit(10) {
        bottom = find_number(input, i + 1, j);
        found = true;
    }

    if bottom_left == bottom {
        bottom_left = 1;
    }

    if i + 1 < input.len() && j + 1 < input[0].len() && input[i + 1][j + 1].is_digit(10) {
        bottom_right = find_number(input, i + 1, j + 1);
        found = true;
    }

    if bottom_left == bottom_right {
        bottom_left = 1;
    }

    if bottom == bottom_right {
        bottom = 1;
    }

    let mut total_count: u32 = 0;

    if top_left != 1 {
        total_count += 1;
    }

    if top != 1 {
        total_count += 1;
    }

    if top_right != 1 {
        total_count += 1;
    }

    if left != 1 {
        total_count += 1;
    }

    if right != 1 {
        total_count += 1;
    }

    if bottom_left != 1 {
        total_count += 1;
    }

    if bottom != 1 {
        total_count += 1;
    }

    if bottom_right != 1 {
        total_count += 1;
    }

    if !found || total_count < 2 {
        return Err("Two gears not found");
    }

    return Ok(top_left * top * top_right * left * right * bottom_left * bottom * bottom_right);
}

pub fn process() -> u32 {
    // Your function implementation
    let filename = "src/day3/input.text";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut input: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        input.push(row)
    }

    let mut sum: u32 = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '*' {
                match near_symbol(&input, i, j) {
                    Ok(number) => {
                        sum += number;
                    }
                    Err(_) => {}
                }
            }
        }
    }

    return sum;
}
