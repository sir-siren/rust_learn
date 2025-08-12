use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
struct NumberIsUnimpressiveError;

impl Display for NumberIsUnimpressiveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "That number is just too small")
    }
}

impl Error for NumberIsUnimpressiveError {}

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let file_read = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => return Err(Box::new(error)),
    };

    let number = match file_read.trim().parse::<i32>() {
        Ok(number) => number,
        Err(error) => return Err(Box::new(error)),
    };

    if number < 100 {
        Err(Box::new(NumberIsUnimpressiveError))
    } else {
        Ok(number)
    }
}

fn main() {
    match read_number_from_file("value.txt") {
        Ok(value) => println!("The number is {value}"),
        Err(error) => println!("The error is {error}"),
    }
}
