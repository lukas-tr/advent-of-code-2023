use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::{ many1, separated_list1},
    IResult,
};
use std::fmt::{self, Debug, Formatter};
use indicatif::ProgressIterator;

#[derive(PartialEq, Eq, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Spring::Operational => write!(f, "."),
            Spring::Damaged => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    alt((
        map(char('#'), |_| Spring::Damaged),
        map(char('.'), |_| Spring::Operational),
        map(char('?'), |_| Spring::Unknown),
    ))(input)
}

fn parse_springs(input: &str) -> IResult<&str, Vec<Spring>> {
    many1(parse_spring)(input)
}

fn parse_group(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(char(','), map_res(digit1, |s: &str| s.parse::<i64>()))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<Spring>, Vec<i64>)> {
    let (input, springs) = parse_springs(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, group) = parse_group(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, (springs, group)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<Spring>, Vec<i64>)>> {
    many1(parse_line)(input)
}

fn choices(line: (Vec<Spring>, Vec<i64>)) -> i64 {
    let (springs, groups) = line;
    let mut choice_count = 0;
    // replace the first unknown spring with a damaged and then with an operational one, and
    // recurse
    for (i, spring) in springs.iter().enumerate() {
        match spring {
            Spring::Unknown => {
                let mut springs = springs.clone();
                springs[i] = Spring::Damaged;
                choice_count += choices((springs.clone(), groups.clone()));
                let mut springs = springs.clone();
                springs[i] = Spring::Operational;
                choice_count += choices((springs, groups.clone()));
                return choice_count;
            }
            _ => {}
        }
    }

    if groups_valid(springs.clone(), groups.clone()) {
        let springs = springs.iter().map(|s| format!("{:?}", s)).collect::<Vec<String>>().join("");
        1
    } else {
        0
    }
}

fn groups_valid(springs: Vec<Spring>, groups: Vec<i64>) -> bool {
    let mut springs = springs.iter();
    let mut groups = groups.iter();
    let mut group = None;
    for spring in springs {
        match (spring, group) {
            (Spring::Unknown, _) => {
                return false
            },
            (Spring::Damaged, Some(g)) => {
                if g == 0 {
                    return false;
                } else {
                    group = Some(g - 1);
                }
            }
            (Spring::Damaged, None) => {
                match groups.next() {
                    Some(g) => {
                        group = Some(g - 1);
                        if group < Some(0) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
            (Spring::Operational, Some(g)) => {
                if g == 0 {
                    group = None;
                } else {
                    return false;
                }
            }
            (Spring::Operational, None) => {}
        }
    }

    (group == Some(0) || group == None) && groups.next() == None
}

fn choice_sum(lines: Vec<(Vec<Spring>, Vec<i64>)>) -> i64 {
    lines.into_iter().map(|l| choices(l)).progress().sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (rest, lines) = parse_input(&input).unwrap();
    let sum = choice_sum(lines);
    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let (rest, lines) = parse_input(&input).unwrap();

        assert_eq!(rest, "");
        assert_eq!(choice_sum(lines), 21);
    }

    #[test]
    fn test_single_line() {
        let input = "?###???????? 3,2,1";
        let (rest, line) = parse_line(&input).unwrap();
        assert_eq!(10, choices(line));
    }

    #[test]
    fn test_single_line_2() {
        let input = "????.######..#####. 1,6,5";
        let (rest, line) = parse_line(&input).unwrap();
        assert_eq!(4, choices(line));
    }

    #[test]
    fn test_groups_valid() {
        let groups = vec![3,2,1];
        let springs = vec![Spring::Operational, Spring::Damaged, Spring::Damaged, Spring::Damaged, Spring::Operational, Spring::Damaged, Spring::Operational, Spring::Damaged, Spring::Operational, Spring::Damaged, Spring::Operational, Spring::Operational];
        assert_eq!(groups_valid(springs, groups), false);
    }

}
