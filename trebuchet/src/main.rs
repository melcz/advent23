use std::fs;
use std::str::Split;
// use std::collections::HashMap;

fn main() {
    println!("Hello 🌍!");

    let input_path = "./input/final.txt";

    println!("In file {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Should have been able to read the file");
    let parts = contents.split("\n");

    // basic(parts.clone());
    spelt(parts.clone());
    
}

// In test, the calibration values of these four lines are 
// 12, 38, 15, and 77. Adding these together produces 142.
fn basic(parts : Split<'_, &str>) {
    let mut calibration_values = Vec::new();

    for part in parts {
        let digits = part.chars().filter(|symbol| symbol.is_ascii_digit());
        
        if digits.clone().count() == 0 {
            println!("Error in {}, no digits found T.T", part);
            return;
        }
        
        let first = digits.clone().next().unwrap();
        let last = digits.clone().last().unwrap();

        let calibration_value = first.to_digit(10).unwrap()*10 + last.to_digit(10).unwrap();
        println!("{}", calibration_value);

        calibration_values.push(calibration_value);
    }

    let sum = calibration_values.iter().fold(0, |acc, x| acc + x); 
    println!("{}", sum);
}

// solves the second part
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. 
// Adding these together produces 281.

// this could be done with a regex, but that would be evil. Or would it?
fn spelt(lines : Split<'_, &str>) {
    let spelt_digits = 
     &[("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)];
    let mut digits = vec![("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), 
        ("6", 6), ("7", 7), ("8", 8), ("9", 9)];

    digits.extend_from_slice(spelt_digits);
    let mut calibration_values = Vec::new();

    for line in lines {
        let mut index_first = line.len() + 1;
        let mut index_last = 0;

        let mut value_first = 0;
        let mut value_last = 0;

        let digits_iter = digits.iter();

        for (spelt_digit, value) in digits_iter {
            let mut occurrences: Vec<_> = line.match_indices(spelt_digit)
                    .map(|(index, _)| index).collect();
            occurrences.sort();

            if occurrences.len() > 0 {
                if let Some(index) = occurrences.first() {
                    if *index <= index_first {
                        index_first = *index;
                        value_first = *value;
                    }
                }

                if let Some(index) = occurrences.last() {
                    if *index >= index_last {
                        index_last = *index;
                        value_last = *value;
                    }
                }
            }
        }

        let calibration_value = value_first*10 + value_last;
        println!("{}", calibration_value);
        calibration_values.push(calibration_value);
    }

    let sum = calibration_values.iter().fold(0, |acc, x| acc + x); 
    println!("{}", sum);
}