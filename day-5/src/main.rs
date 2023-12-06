use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Vec<Mapping>>,
}

#[derive(Debug, Clone)]
struct Mapping {
    from: u64,
    to: u64,
    count: u64,
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().replace("seeds: ", "");
    let seeds = first_line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut mappings = Vec::new();
    let mut group = Vec::new();
    for line in lines {
        if line.contains("map:") {
            if group.len() > 0 {
                mappings.push(group);
                group = Vec::new();
            }
        } else if line == "" {
            continue;
        } else {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let from = parts[1].parse::<u64>().unwrap();
            let to = parts[0].parse::<u64>().unwrap();
            let count = parts[2].parse::<u64>().unwrap();
            group.push(Mapping { from, to, count });
        }
    }
    mappings.push(group);

    Almanac {
        seeds,
        mappings,
    }
}

fn get_lowest_location_number(almanac: &Almanac) -> u64 {
    let locations = almanac.seeds.par_iter().map(|seed| {
        let mut id = *seed;
        for group in &almanac.mappings {
            for mapping in group {
                if id >= mapping.from && id < mapping.from + mapping.count {
                    id = mapping.to + (id - mapping.from);
                    break;
                }
            }
        }
        id
    });

    locations.min().unwrap()
}

fn convert_seed_range(almanac: &Almanac) -> Almanac {
    let chunks = almanac.seeds.clone().into_iter().chunks(2).into_iter().map(|chunk| chunk.collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>();
    let seeds = chunks.par_iter().flat_map(|chunk| {
        let start = chunk[0];
        let length = chunk[1];
        (start..start + length).collect::<Vec<u64>>()
    }).collect::<Vec<u64>>();

    Almanac {
        seeds,
        mappings: almanac.mappings.clone(),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let parsed = parse_input(&input);
    let lowest_location_number = get_lowest_location_number(&parsed);
    println!("lowest location number: {}", lowest_location_number);

    let parsed = convert_seed_range(&parsed);
    let lowest_location_number = get_lowest_location_number(&parsed);
    println!("lowest location number (range): {}", lowest_location_number);
}



