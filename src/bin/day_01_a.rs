use std::env::args;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];
    let file = read_to_string(file_path).unwrap();

    let result = find_calibration_sum(&file);
    match result {
        Some(result) => println!("{}", result),
        None => println!("Something went wrong!"),
    }
}

fn find_calibration_sum(file: &str) -> Option<u32> {
    file.lines().map(find_calibration).sum()
}

fn find_calibration(line: &str) -> Option<u32> {
    let first = find_digit(line.chars())?;
    let last = find_digit(line.chars().rev())?;
    Some(first * 10 + last)
}

fn find_digit(vals: impl Iterator<Item = char>) -> Option<u32> {
    vals.into_iter().find_map(|c | c.to_digit(10))
}
