// winning numbers | and eight numbers you have
// Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! 
// That means card 1 is worth 8 points (1 for the first match, then doubled three times 
//     for each of the three matches after the first).
//     So, in this example, the Elf's pile of scratchcards is worth 13 points.

use std::fs;
use std::collections::{HashSet, HashMap};
use core::str::Lines;

fn main() {
    let input_path = "./input.txt";

    println!("In file {}", input_path);

    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    // scratchcard_points(lines);
    calculate_won_scratchcards(lines);
}

fn scratchcard_points(lines: Lines) {
    let mut points: u32 = 0;
    let base: u32 = 2;
    for card in lines {
        let numbers = skip_header(card);

        let (winning_numbers, drawn_numbers) = numbers.split_once(" | ").unwrap();
        let winner_set: HashSet<&str> = winning_numbers.split(" ").filter(|x| !(*x).is_empty()).collect();
        let drawn_set: HashSet<&str> = drawn_numbers.split(" ").filter(|x| !(*x).is_empty()).collect();
        let lotería: HashSet<&&str> = winner_set.intersection(&drawn_set).collect();

        match lotería.len() {
            0 => (),
            1 => points += 1,
            more => points += base.pow((more - 1).try_into().unwrap()),
        }

        // println!("{:?}  {:?}", winner_set, drawn_set);
        // println!("{:?}", lotería);
    }

    println!("Lotería {}", points);
}

// 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4,
// 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards
// causes you to ultimately have 30
fn calculate_won_scratchcards(lines: Lines) {
    let mut current: usize = 0;
    let mut scratchcards: Vec<usize> = vec![1; lines.clone().count()];
    for card in lines {
        let numbers = skip_header(card);

        let (winning_numbers, drawn_numbers) = numbers.split_once(" | ").unwrap();
        let winner_set: HashSet<&str> = winning_numbers.split(" ").filter(|x| !(*x).is_empty()).collect();
        let drawn_set: HashSet<&str> = drawn_numbers.split(" ").filter(|x| !(*x).is_empty()).collect();
        let lotería: HashSet<&&str> = winner_set.intersection(&drawn_set).collect();

        if lotería.len() == 0 {
            current = current + 1;
            continue;
        }

        let copies = scratchcards[current];

        for copy_idx in (1..lotería.len() + 1) {
            if (current + copy_idx < scratchcards.len()) {
                scratchcards[current + copy_idx] = scratchcards[current + copy_idx] + copies;
            }
        }

        current = current + 1;
    }

    let total_scratchcards = scratchcards.iter().fold(0, |acc, x| acc + x);
    println!("Total scratchcards {}", total_scratchcards);
}

fn skip_header(card: &str) -> &str {
    let (header, numbers) = card.split_once(": ").unwrap();
    return numbers;
}
