fn parse_sequences(input: String) -> Vec<Vec<i64>> {
    input.lines().map(|line| line.split(" ").map(|item| item.parse::<i64>().unwrap()).collect()).collect()
}

fn extrapolate_sequence(sequence: &[i64]) -> Vec<i64> {
    let differences = sequence.windows(2).map(|window| window[1] - window[0]).collect::<Vec<i64>>();
    let next_sequence = if differences.iter().all(|i| *i == 0) {
        differences.iter().chain([&0].into_iter()).cloned().collect()
    } else {
        extrapolate_sequence(&differences)
    };
    let cur_last = sequence.last().unwrap();
    let next_last = next_sequence.last().unwrap();
    let res: Vec<i64> = sequence.iter().copied().chain([cur_last + next_last].into_iter()).collect();
    res
}

fn extrapolate_sequences(sequences: Vec<Vec<i64>>) -> i64 {
    sequences.iter().map(|sequence| extrapolate_sequence(sequence).last().unwrap().clone()).sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sequences = parse_sequences(input);
    let sum = extrapolate_sequences(sequences.clone());
    println!("Extrapolated last sum: {:?}", sum);

    let sequences = sequences.iter().map(|s| s.iter().cloned().rev().collect()).collect();
    let sum = extrapolate_sequences(sequences);
    println!("Extrapolated first sum: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string();
        let sequences = parse_sequences(input);
        let sum = extrapolate_sequences(sequences);
        assert_eq!(114, sum);
    }

    #[test]
    fn test_first() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string();
        let sequences = parse_sequences(input);

        let sequences = sequences.iter().map(|s| s.iter().cloned().rev().collect()).collect();
        let sum = extrapolate_sequences(sequences);
        assert_eq!(2, sum);
    }
}

