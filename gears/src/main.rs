use core::str::Lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input_path = "./input.txt";

    println!("In file {}", input_path);

    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    // sum_schematic(lines);
    verify_gears(lines);
}

fn sum_schematic(lines: Lines) {
    let matrix = build_matrix(lines);
    // print_matrix(&matrix);

    let mut part_numbers = Vec::new();
    let mut digits = Vec::new();
    let mut adjacent = false;

    for row in 0..matrix.len() {
        for column in 0..matrix[0].len() {
            let cell_char = matrix[row][column];
            if cell_char.is_ascii_digit() {
                digits.push(cell_char);
                if !adjacent {
                    adjacent |= check_adjacent_symbol(&matrix, row, column);
                }

            // found a symbol after some digits
            } else if !digits.is_empty() {
                if !adjacent {
                    adjacent |= check_column_neighbors(&matrix, row, column);
                }
                let part_number: String = digits.iter().collect();
                if adjacent {
                    part_numbers.push(part_number.parse::<u32>().unwrap());
                }
                digits.clear();
                adjacent = false;
            }
        }

        if !digits.is_empty() && adjacent {
            let part_number: String = digits.iter().collect();
            part_numbers.push(part_number.parse::<u32>().unwrap());
            digits.clear();
            adjacent = false;
        }
    }

    // println!("{:?}", part_numbers);
    let sum = part_numbers.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

// checks if there is a symbol in the given cell, above or below or in the column to the left
fn check_adjacent_symbol(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if check_column_neighbors(matrix, row, column) {
        return true;
    }
    if column > 0 && check_column_neighbors(matrix, row, column - 1) {
        return true;
    }
    return false;
}

// checks if there is a symbol in the given cell, above or below
fn check_column_neighbors(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    if row > 0 && is_symbol(matrix[row - 1][column]) {
        return true;
    }
    if row < matrix.len() - 1 && is_symbol(matrix[row + 1][column]) {
        return true;
    }
    if is_symbol(matrix[row][column]) {
        return true;
    }

    return false;
}

fn is_symbol(cell: char) -> bool {
    match cell {
        cell if cell.is_ascii_digit() => return false,
        '.' => return false,
        _ => return true,
    }
}

fn verify_gears(lines: Lines) {
    let matrix = build_matrix(lines);
    // print_matrix(&matrix);

    let mut geared_parts: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut digits = Vec::new();
    let mut adjacent_gears: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..matrix.len() {
        for column in 0..matrix[0].len() {
            let cell_char = matrix[row][column];
            if cell_char.is_ascii_digit() {
                digits.push(cell_char);
                add_adjacent_gear_positions(&matrix, row, column, &mut adjacent_gears);

            // found a symbol after some digits
            } else if !digits.is_empty() {
                add_adjacent_gear_positions(&matrix, row, column, &mut adjacent_gears);
                let part_number: String = digits.iter().collect();
                register_geared_part(
                    &mut geared_parts,
                    part_number.parse::<u32>().unwrap(),
                    &adjacent_gears,
                );
                digits.clear();
                adjacent_gears.clear();
            }
        }

        if !digits.is_empty() {
            let part_number: String = digits.iter().collect();
            register_geared_part(
                &mut geared_parts,
                part_number.parse::<u32>().unwrap(),
                &adjacent_gears,
            );
            digits.clear();
            adjacent_gears.clear();
        }
    }

    println!("{:?}", geared_parts);
    let sum = geared_parts
        .iter()
        .filter(|(gear_pos, parts)| parts.len() == 2)
        .map(|(gear_pos, parts)| parts[0] * parts[1])
        .fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

fn register_geared_part(
    geared_parts: &mut HashMap<(usize, usize), Vec<u32>>,
    part_number: u32,
    adjacent_gears: &HashSet<(usize, usize)>,
) {
    for adjacent_gear_pos in adjacent_gears {
        if let None = geared_parts.get(&adjacent_gear_pos).as_mut() {
            let parts: Vec<u32> = Vec::new();
            geared_parts.insert(*adjacent_gear_pos, parts);
        }

        let geared_parts_vec: &mut Vec<u32> = geared_parts.get_mut(&adjacent_gear_pos).unwrap();
        geared_parts_vec.push(part_number);
    }
}

fn add_adjacent_gear_positions(
    matrix: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    adjacent_gears: &mut HashSet<(usize, usize)>,
) {
    if row > 0 && matrix[row - 1][column] == '*' {
        adjacent_gears.insert((row - 1, column));
    }
    if row < matrix.len() - 1 && matrix[row + 1][column] == '*' {
        adjacent_gears.insert((row + 1, column));
    }
    if row < matrix.len() - 1 && matrix[row][column] == '*' {
        adjacent_gears.insert((row, column));
    }

    if column > 0 {
        if row > 0 && matrix[row - 1][column - 1] == '*' {
            adjacent_gears.insert((row - 1, column - 1));
        }
        if row < matrix.len() - 1 && matrix[row + 1][column - 1] == '*' {
            adjacent_gears.insert((row + 1, column - 1));
        }
        if matrix[row][column - 1] == '*' {
            adjacent_gears.insert((row, column - 1));
        }
    }
}

fn build_matrix(lines: Lines) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for row in lines {
        matrix.push(row.chars().collect());
    }
    return matrix;
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
}

// Two numbers are not part numbers because they are not adjacent to a symbol:
// 114 (top right) and 58 (middle right).
// Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
