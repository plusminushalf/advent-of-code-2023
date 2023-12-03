use std::fs;

pub fn process() {
    // Your function implementation
    let filename = "src/day3/test.text";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{contents}");

}
