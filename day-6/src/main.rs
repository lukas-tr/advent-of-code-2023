
fn parse_races(input: String) -> Vec<(i64, i64)> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().replace("Time:", "").split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let distances = lines.next().unwrap().replace("Distance:", "").split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    times.iter().zip(distances.iter()).map(|(t, d)| (*t, *d)).collect::<Vec<(i64, i64)>>()
}

fn calculate_records(races: Vec<(i64, i64)>) -> i64 {
    races.iter().map(|(time, distance)| 
        (1..*time).map(|time_held_down| {
            let final_distance = (time - time_held_down) * time_held_down;
            final_distance
        }).filter(|final_distance| {
            *final_distance > *distance
        }).count() as i64
    ).product::<i64>()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let races = parse_races(input.clone());
    let records = calculate_records(races);
    println!("Part 1: {:?}", records);

    let races = parse_races(input.replace(" ", ""));
    let records = calculate_records(races);
    println!("Part 2: {:?}", records);
}

