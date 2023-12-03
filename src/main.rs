mod day3 {
    pub mod process;
}

use day3::process::process;

fn main() {

    let result: u32 = process();

    print!("Sum: {}", result);
    println!()
}
