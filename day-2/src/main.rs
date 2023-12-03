use std::fs::File;
use std::io::prelude::*;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::branch::alt;
use nom::multi::separated_list1;
use nom::character::complete::space0;

fn read_input_file(path: &str ) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

#[derive(Debug, PartialEq, Clone)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

fn games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(tag("\n"), game)(input)?;
    Ok((input, games))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = space0(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, reveals) = separated_list1(tag(";"), reveal)(input)?;
    Ok((input, Game { id, reveals }))
}

fn reveal(input: &str) -> IResult<&str, Reveal> {
    let (input, colors) = separated_list1(tag(","), color)(input)?;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for (color, count) in colors {
        match color {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            _ => panic!("Unknown color: {}", color),
        }
    }
    Ok((input, Reveal { red, green, blue }))
}

fn color(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, _) = space0(input)?;
    let (input, count) = u32(input)?;
    let (input, _) = space0(input)?;
    let (input, color) = alt((
        tag("red"),
        tag("green"),
        tag("blue"),
    ))(input)?;
    Ok((input, (color, count)))
}

fn possible_game_sum(games: Vec<Game>, total: Reveal) -> u32 {
    games
        .iter()
        .filter(|game| {
            game.reveals.iter().all(|reveal|
                                    reveal.red <= total.red &&
                                    reveal.green <= total.green &&
                                    reveal.blue <= total.blue
                                   )
            })
        .map(|game| game.id)
        .sum::<u32>()
}

fn game_power_sum(games: Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let mut min_reveal = game.reveals[0].clone();
            for reveal in &game.reveals {
                if reveal.red > min_reveal.red {
                    min_reveal.red = reveal.red;
                }
                if reveal.green > min_reveal.green {
                    min_reveal.green = reveal.green;
                }
                if reveal.blue > min_reveal.blue {
                    min_reveal.blue = reveal.blue;
                }
            }
            min_reveal.red * min_reveal.green * min_reveal.blue
        })
        .sum::<u32>()
}

fn main() {
    let total = Reveal { red: 12, green: 13, blue: 14 };
    let input = read_input_file("input.txt");
    let (_, games) = games(&input).unwrap();

    let id_sum = possible_game_sum(games.clone(), total);
    println!("Sum of game IDs: {}", id_sum);

    let power_sum = game_power_sum(games);
    println!("Sum of game powers: {}", power_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let parsed = games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue");
        let expected = Ok(("", vec![
            Game {
                id: 1,
                reveals: vec![
                    Reveal { red: 4, green: 0, blue: 3 },
                    Reveal { red: 1, green: 2, blue: 6 },
                    Reveal { red: 0, green: 2, blue: 0 },
                ],
            },
            Game {
                id: 2,
                reveals: vec![
                    Reveal { red: 0, green: 0, blue: 1 },
                ],
            },
        ]));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_game_power() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let (_, games) = games(input).unwrap();
        let power_sum = game_power_sum(games);
        assert_eq!(power_sum, 2286);
    }
}

