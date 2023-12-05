use std::fmt::Error;
use std::fs; 
use std::io::prelude::*;


fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let red_line = &line[index..];
        let res = if red_line.starts_with("one") {
            Some('1')
        } else if red_line.starts_with("two") {
            Some('2')
        } else if red_line.starts_with("three") {
            Some('3')
        } else if red_line.starts_with("four") {
            Some('4')
        } else if red_line.starts_with("five") {
            Some('5')
        } else if red_line.starts_with("six") {
            Some('6')
        } else if red_line.starts_with("seven") {
            Some('7')
        } else if red_line.starts_with("eight") {
            Some('8')
        } else if red_line.starts_with("nine") {
            Some('9')
        } else {
            let res = red_line.chars().next();
            res
        };
        index += 1;
        res
    });
    let mut it = line_iter.filter_map(|character| { character.to_digit(10) });
    let first = it.next().expect("should be num");
    let last = it.last();
    let result = match last {
        Some(num) => format!("{first}{num}").parse::<u32>(),
        None => format!("{first}{first}").parse::<u32>(),
    };

    match result {
        Ok(num) => num,
        Err(_) => 0,
    }
}


fn process(input: &str) -> Result<u32, Error> {
    let output: u32 = input
        .lines()
        .map(process_line).sum();

    Ok(output)
    //         |line| {
    //         let line = line
    //             .replace("one", "1")
    //             .replace("two", "2")
    //             .replace("three", "3")
    //             .replace("four", "4")
    //             .replace("five", "5")
    //             .replace("six", "6")
    //             .replace("seven", "7")
    //             .replace("eight", "8")
    //             .replace("nine", "9");
    //         println!("New line: {:#?}", line);
    //     let mut it = line.chars().filter_map(|character| {
    //         character.to_digit(10)
    //     });
    //     let first = it.next().expect("should be num");
    //     let last = it.last();
    //     let result = match last {
    //         Some(num) => format!("{first}{num}").parse::<u32>(),
    //         None => format!("{first}{first}").parse::<u32>(),
    //     };

    //     match result {
    //         Ok(num) => Ok(num),
    //         Err(_) => Err(Error),
    //     }
    // });

    // let valid_numbers: Result<Vec<u32>, Error> = output.collect();
    // let sum: u32 = valid_numbers?.iter().sum();

    // Ok(output)
}

fn main() -> std::io::Result<()> {
    let filepath = "src/input_text.txt";
    let mut file = fs::File::open(filepath)?;
    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents);

    let mut total_sum = 0;

    match process(&contents) {
        Ok(val) => {
            println!("Val: {:?}", val);
            total_sum = val;
        }
        Err(e) => eprintln!("Error: {:?}", e), 
       };

    // let lines = contents.lines();
    // for (i, line) in lines.enumerate() {
    //     if let Some(indx) = line.find("eight") {
    //         println!("{0}: {1}", indx, line);
    //     }
    // }
    println!("Total sum: {:?}", total_sum);

    Ok(())
}
