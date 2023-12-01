use std::fs::File;
use std::io::prelude::*;

fn read_input_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

fn calculate_sum(input: String) -> u32 {
    input.split("\n").into_iter().map(|line| {
        let first_digit = line.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10)).next();
        let last_digit = line.chars().rev().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10)).next();
        match (first_digit, last_digit) {
            (Some(Some(first)), Some(Some(last))) => {
                first * 10 + last
            },
            _ => 0
        }
    }).map(|num| {
        println!("{}", num);
        num
    }).sum::<u32>()
}

// find the first spelled out number (like one, two, three, etc)
fn find_first_number(input: &str, order: impl Iterator<Item = usize>) -> u32 {
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for i in order {
        let cur_char = input.chars().nth(i).unwrap();
        if let Some(digit) = cur_char.to_digit(10) {
            return digit;
        }
        let slice = &input[i..];
        for j in 0..numbers.len() {
            if slice.starts_with(numbers[j]) {
                return j as u32;
            }
        }
    }
    0
}

fn calculate_sum_spelled(input: String) -> u32 {
    input.split("\n").into_iter().filter(|line| line.len() > 1).map(|line| {
        let first = find_first_number(line, directed_range(0, line.len() - 1));
        let last = find_first_number(line, directed_range(line.len() - 1, 0));
        first * 10 + last
    }).map(|num| {
        println!("{}", num);
        num
    }).sum::<u32>()
}

fn main() {
    let input = read_input_file("part_1.txt");
    let sum = calculate_sum(input);
    println!("Sum (part 1): {}", sum);

    let input = read_input_file("part_2.txt");
    let sum = calculate_sum_spelled(input);
    println!("Sum (part 2): {}", sum);
    // 53655 is too low
}

fn directed_range(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let mut start = a;
    let end = b;
    let mut done = false;
    std::iter::from_fn(move || {
            use std::cmp::Ordering::*;
            match start.cmp(&end) {
                Less => {
                    start += 1;
                    Some(start - 1)
                }
                Equal => {
                    if done {
                        None
                    } else {
                        done = true;
                        Some(end)
                    }
                },
                Greater => {
                    start -= 1;
                    Some(start + 1)
                }
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spelled_reverse() {
        let input = "two1nine";
        let last = find_first_number(input, directed_range(input.len() - 1, 0));
        assert_eq!(last, 9);
    }

    #[test]
    fn test_directed_range() {
        let mut range = directed_range(3, 0);
        assert_eq!(range.next(), Some(3));
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next(), Some(1));
        assert_eq!(range.next(), Some(0));
    }

    #[test]
    fn test_spelled_single_digit_first() {
        let input = "4bx";
        let sum = calculate_sum_spelled(input.to_string());
        assert_eq!(sum, 44);
    }

    #[test]
    fn test_spelled_single_digit_last() {
        let input = "xb4";
        let sum = calculate_sum_spelled(input.to_string());
        assert_eq!(sum, 44);
    }

    #[test]
    fn test_spelled_first_last() {
        let input = "1asdf9";
        let sum = calculate_sum_spelled(input.to_string());
        assert_eq!(sum, 19);
    }
}

