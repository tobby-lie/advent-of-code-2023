use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SPELLED_DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

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
    let mut first_digit_found: bool = false;
    let mut last_digit_found: bool = false;
    let mut combined_digits: u32 = 0;
    for j in 0..s.len() {
        if first_digit_found && last_digit_found {
            break;
        }
        for (i, &spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
            if !first_digit_found {
                if s[j..].starts_with(spelled_digit) {
                    combined_digits += (i as u32 + 1) * 10;
                    first_digit_found = true;
                }
                else if let Some(c) = s.chars().nth(j) {
                    if c.is_ascii_digit() {
                        let digit = c.to_digit(10).unwrap();
                        combined_digits += (digit * 10) as u32;
                        first_digit_found = true;
                    }
                }

            }
            if !last_digit_found {
                if s[..s.len() - j].ends_with(spelled_digit) {
                    combined_digits += i as u32 + 1;
                    last_digit_found = true;
                }
                else if let Some(c) = s.chars().nth(s.len() - (j+1)) {
                    if c.is_ascii_digit() {
                        let digit = c.to_digit(10).unwrap();
                        combined_digits += digit as u32;
                        last_digit_found = true;
                    }
                }
            }
        }
    }
    combined_digits
}
