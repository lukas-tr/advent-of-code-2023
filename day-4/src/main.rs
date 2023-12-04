use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use pest_derive::Parser;
use pest::Parser;

fn read_input_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

#[derive(Parser)]
#[grammar = "cards.pest"]
struct CardsParser;

fn parse_number_list(pair: pest::iterators::Pair<Rule>) -> HashSet<u8> {
    let mut numbers = HashSet::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::number_list => {
                for inner in inner.into_inner() {
                    match inner.as_rule() {
                        Rule::number => {
                            numbers.insert(inner.as_str().parse::<u8>().unwrap());
                        }
                        _ => {},
                    }
                }
            }
            _ => {},
        }
    }
    numbers
}

fn calculate_sum(cards: pest::iterators::Pairs<Rule>) -> i32 {
    let mut sum = 0;

    for card in cards {
        for inner in card.into_inner() {
            match inner.as_rule() {
                Rule::card => {
                    let mut winning_numbers = HashSet::new();
                    let mut own_numbers = HashSet::new();
                    for inner in inner.into_inner() {
                        match inner.as_rule() {
                            Rule::winning_numbers => {
                                winning_numbers = parse_number_list(inner);
                            }
                            Rule::own_numbers => {
                                own_numbers = parse_number_list(inner);
                            }
                            _ => {},
                        }
                    }
                    let result = winning_numbers.intersection(&own_numbers);
                    let count = result.clone().count();
                    let points = if count == 0 {
                        0
                    } else {
                        2_i32.pow(<usize as TryInto<u32>>::try_into(count).unwrap() - 1)
                    };
                    sum += points;
                }
                _ => unreachable!(),
            }
        }
    }

    sum
}

fn calculate_cards(cards: pest::iterators::Pairs<Rule>) -> i32 {
    let mut card_counts = HashMap::new();

    for card in cards {
        for inner in card.into_inner() {
            match inner.as_rule() {
                Rule::card => {
                    let mut card_number = 0;
                    let mut winning_numbers = HashSet::new();
                    let mut own_numbers = HashSet::new();
                    for inner in inner.into_inner() {
                        match inner.as_rule() {
                            Rule::winning_numbers => {
                                winning_numbers = parse_number_list(inner);
                            }
                            Rule::own_numbers => {
                                own_numbers = parse_number_list(inner);
                            }
                            Rule::number => {
                                card_number = inner.as_str().parse::<u8>().unwrap();
                            }
                            _ => {},
                        }
                    }
                    card_counts.entry(card_number).or_insert(1);
                    let result = winning_numbers.intersection(&own_numbers);
                    let copies = result.clone().count();
                    let this_card_count = {
                        let card_counts = card_counts.clone();
                        card_counts.get(&card_number).unwrap_or(&1).to_owned()
                    };
                    for card_number in card_number + 1..card_number + copies as u8 + 1 {
                        let count = card_counts.entry(card_number).or_insert(1);
                        *count += this_card_count;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    card_counts.values().sum()
}

fn main() {
    let input = read_input_file("input.txt");
    let cards = CardsParser::parse(Rule::cards, &input).unwrap_or_else(|e| panic!("{}", e));
    let sum = calculate_sum(cards.clone());
    println!("Sum: {}", sum);

    let cards = calculate_cards(cards);
    println!("Cards: {}", cards);
}

