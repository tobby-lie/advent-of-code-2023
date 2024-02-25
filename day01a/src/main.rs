use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines("./calibration_document.txt") {
        for line in lines.flatten() {
            total += combine_first_last_digits(&line);
        }
    }
    println!("total -> {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn combine_first_last_digits(s: &String) -> u32 {
    let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
    let digits_len: usize = digits.len();
    let combined_digits: String = format!("{}{}", digits[0], digits[digits_len - 1]);
    combined_digits.parse::<u32>().unwrap()
}
