use itertools::Itertools;

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u8>,
    bid: i64,
}

impl Hand {
    fn hand_type(&self) -> u8 {
        let cards = &self.cards.clone()
            .into_iter()
            .filter(|&card| card != 1)
            .sorted()
            .group_by(|&card| card)
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .rev()
            .collect::<Vec<_>>();
        match cards.as_slice() {
            // with (<5 cards) and without (5 cards) joker
            [] | [1] | [2] | [3] | [4] | [5] => 7,
            [1, 1] | [2, 1] | [3, 1] | [4, 1] => 6,
            [3, 2] | [2, 2] => 5,
            [3, 1, 1] | [2, 1, 1] | [1, 1, 1] => 4,
            [2, 2, 1] => 3,
            [2, 1, 1, 1] | [1, 1, 1, 1] => 2,
            [1, 1, 1, 1, 1] => 1,
            _ => panic!("Invalid hand: {:?}", cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type = self.hand_type();
        let other_hand_type = other.hand_type();
        if hand_type == other_hand_type {
            self.cards.clone().into_iter().zip(other.cards.clone().into_iter()).find_map(|(a, b)| {
                if a != b {
                    Some(a.cmp(&b))
                } else {
                    None
                }
            }).unwrap()
        } else {
            return hand_type.cmp(&other_hand_type);
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

fn parse_input(input: &str, joker: bool) -> Vec<Hand> {
    input.lines().map(|line| {
        let mut parts = line.split(" ");
        let cards = parts.next().unwrap();
        let cards = cards.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if joker { 1 } else { 11 },
                'T' => 10,
                _ => c.to_digit(10).unwrap() as u8,
            }
        }).collect();
        let bid = parts.next().unwrap().parse().unwrap();
        Hand { cards, bid }
    }).collect()
}

fn calculate_winnings(hands: &[Hand]) -> i64 {
    hands.into_iter().sorted().enumerate().map(|(pos, hand)| {
        println!("Hand: {:?} - pos {} - type {}", hand, pos, hand.hand_type());
        hand.bid * (pos + 1) as i64
    }).sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let hands = parse_input(&input, false);
    let result = calculate_winnings(&hands);
    println!("Winnings: {}", result);

    let hands = parse_input(&input, true);
    let result = calculate_winnings(&hands);
    println!("Winnings: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "AAAAA 1\nKAKKK 10";
        let hands = parse_input(&input, false);
        let result = calculate_winnings(&hands);
        assert_eq!(result, 10 * 1 + 1 * 2);
    }

    #[test]
    fn test_2() {
        let input = "2342A 1\n2343A 10";
        let hands = parse_input(&input, false);
        let result = calculate_winnings(&hands);
        assert_eq!(result, 1 * 1 + 10 * 2);
    }
}
