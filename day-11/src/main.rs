use itertools::Itertools;


#[derive(Debug, PartialEq)]
struct Galaxy {
    row: i64,
    col: i64,
}

impl Galaxy {
    fn dist(&self, other: &Galaxy) -> i64 {
        (self.row.abs_diff(other.row) + self.col.abs_diff(other.col)) as i64
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn expand_empty_lines(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.into_iter().flat_map(|row| if row.iter().all(|has_galaxy| !has_galaxy) { vec![row.clone(), row] } else { vec![row] }).collect()
}

fn find_galaxies(input: &str) -> Vec<Galaxy> {
    let grid: Vec<Vec<bool>> = input.lines().map(| row | row.chars().map(|char| char == '#').collect()).collect();
    let grid = transpose(expand_empty_lines(transpose(expand_empty_lines(grid))));

    grid.into_iter().enumerate().flat_map(|(row, rows)| rows.into_iter().enumerate().filter_map(move |(col, is_galaxy)| if is_galaxy { Some(Galaxy{row: (row as i64).clone(), col: col as i64}) } else { None })).collect()
}

fn sum_paths(galaxies: Vec<Galaxy>) -> i64 {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.dist(b)).sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");
    let galaxies = find_galaxies(&input);
    println!("Distance sum: {}", sum_paths(galaxies));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();
        let galaxies = find_galaxies(&input);
        let result = sum_paths(galaxies);

        assert_eq!(result, 374);
    }
}

