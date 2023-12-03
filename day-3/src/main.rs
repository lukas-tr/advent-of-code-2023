use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn read_input_file(path: &str ) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn sum_parts(input: Vec<Vec<char>>) -> (u32, u32) {
    let mut sum = 0;
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for row in 0..input.len() {
        let mut current_part_number = 0;
        let mut current_part_number_touches_symbol = false;
        let mut gears_touched = HashSet::new();
        for col in 0..input[row].len() {
            let digit = input[row][col].to_digit(10);
            if let Some(digit) = digit {
                current_part_number = current_part_number * 10 + digit;

                for row in row.checked_sub(1).unwrap_or(0)..=row+1 {
                    for col in col.checked_sub(1).unwrap_or(0)..=col+1 {
                        if row >= input.len() || col >= input[row].len() {
                            continue;
                        }
                        if !input[row][col].is_digit(10) && input[row][col] != '.' {
                            current_part_number_touches_symbol = true;
                        }
                        if input[row][col] == '*' {
                            gears_touched.insert((row, col));
                        }
                    }
                }
            } else {
                if current_part_number > 0 && current_part_number_touches_symbol {
                    sum += current_part_number;
                    for gear in gears_touched.clone() {
                        gear_map.entry(gear).or_insert(Vec::new()).push(current_part_number as u32);
                    }
                }
                current_part_number = 0;
                current_part_number_touches_symbol = false;
                gears_touched.clear();
            }
        }
        if current_part_number > 0 && current_part_number_touches_symbol {
            sum += current_part_number;
            for gear in gears_touched {
                gear_map.entry(gear).or_insert(Vec::new()).push(current_part_number as u32);
            }
        }
    }
    let gear_ratios = gear_map.iter().fold(0, |sum, (_, ratios)| {
        if ratios.len() == 2 {
            sum + ratios[0] * ratios[1]
        } else {
            sum
        }
    });

    (sum, gear_ratios)
}

fn main() {
    let input = read_input_file("input.txt");
    let parsed = parse_input(&input);
    let (sum, ratio) = sum_parts(parsed);
    println!("Sum of parts: {}", sum);
    println!("Gear ratios: {}", ratio);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_numbers() {
        let input = "467..114.1
1..*.....1
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.1";
        let parsed = parse_input(input);
        assert_eq!(sum_parts(parsed), (4361, 467835));
    }

    #[test]
    fn test_part_numbers2() {
        let input = "
46...114*1
1..*.....1
..3.5..633";
        let parsed = parse_input(input);
        assert_eq!(sum_parts(parsed), (114 + 1 + 1 + 3 + 5, 3 * 5));
    }
}

