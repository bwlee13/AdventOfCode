use std::fmt::Error;
use std::fs::File; 
use std::io::{BufReader, BufRead};

fn process(input: &str) -> Result<u32, Error> {
    let output = input
        .lines()
        .map(|line| {
        let mut it = line.chars().filter_map(|character| {
            character.to_digit(10)
        });
        let first = it.next().expect("should be num");
        let last = it.last();
        let result = match last {
            Some(num) => format!("{first}{num}").parse::<u32>(),
            None => format!("{first}{first}").parse::<u32>(),
        };

        match result {
            Ok(num) => Ok(num),
            Err(_) => Err(Error),
        }
    });

    let valid_numbers: Result<Vec<u32>, Error> = output.collect();
    let sum: u32 = valid_numbers?.iter().sum();

    Ok(sum)
}

fn main() {
    let filepath = "src/input_text.txt";
    let file = File::open(filepath).expect("Expect to open file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut total_sum: u32 = 0;

    for line in lines {
       match process(&line.expect("msg")) {
        Ok(val) => {
            println!("Val: {:?}", val);
            total_sum += val;
        }
        Err(e) => eprintln!("Error: {:?}", e), 
       }
    }
    println!("Total sum: {:?}", total_sum)
}
