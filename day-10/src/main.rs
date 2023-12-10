use std::collections::HashSet;

#[derive(Debug)]
struct Pipes {
    map: Vec<Vec<Pipe>>,
    start: (i64, i64),
}

#[derive(Debug)]
struct Pipe {
    kind: PipeKind,
    position: (i64, i64),
}

#[derive(Debug, PartialEq)]
enum PipeKind {
    None,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    fn new(kind: PipeKind, position: (i64, i64)) -> Pipe {
        Pipe { kind, position }
    }
    fn connects_to_position(&self, other_position: (i64, i64)) -> bool {
        match self.kind {
            PipeKind::None => false,
            PipeKind::Vertical => self.position.1 == other_position.1 && (self.position.0 == other_position.0 - 1 || self.position.0 == other_position.0 + 1),
            PipeKind::Horizontal => self.position.0 == other_position.0 && (self.position.1 == other_position.1 - 1 || self.position.1 == other_position.1 + 1),
            PipeKind::NorthEast => (self.position.1 == other_position.1 - 1 && self.position.0 == other_position.0) || (self.position.1 == other_position.1 && self.position.0 == other_position.0 + 1),
            PipeKind::NorthWest => (self.position.1 == other_position.1 + 1 && self.position.0 == other_position.0) || (self.position.1 == other_position.1 && self.position.0 == other_position.0 + 1),
            PipeKind::SouthWest => (self.position.1 == other_position.1 + 1 && self.position.0 == other_position.0) || (self.position.1 == other_position.1 && self.position.0 == other_position.0 - 1),
            PipeKind::SouthEast => (self.position.1 == other_position.1 - 1 && self.position.0 == other_position.0) || (self.position.1 == other_position.1 && self.position.0 == other_position.0 - 1),
        }
    }
}

impl Pipes {
    fn parse(input: &str) -> Pipes {
        let mut map = Vec::new();
        let mut start = (0, 0);
        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let pipe_kind = match c {
                    '-' => PipeKind::Horizontal,
                    '|' => PipeKind::Vertical,
                    'L' => PipeKind::NorthEast,
                    'J' => PipeKind::NorthWest,
                    '7' => PipeKind::SouthWest,
                    'F' => PipeKind::SouthEast,
                    '.' => PipeKind::None,
                    'S' => {
                        start = (i as i64, j as i64);
                        PipeKind::None
                    }
                    _ => {
                        panic!("Unknown pipe: {}", c);
                    }
                };
                let pipe = Pipe::new(pipe_kind, (i as i64, j as i64));
                row.push(pipe);
            }
            map.push(row);
        }
        let start_kind = if map[start.0 as usize][(start.1 - 1) as usize].connects_to_position((start.0, start.1)) {
            if map[(start.0 - 1) as usize][start.1 as usize].connects_to_position((start.0, start.1)) {
                PipeKind::NorthWest
            } else if map[(start.0 + 1) as usize][start.1 as usize].connects_to_position((start.0, start.1)) {
                PipeKind::SouthWest
            } else {
                PipeKind::Horizontal
            }
        } else if map[(start.0 + 1) as usize][start.1 as usize].connects_to_position((start.0, start.1)) {
            if map[(start.0 - 1) as usize][start.1 as usize].connects_to_position((start.0, start.1)) {
                PipeKind::NorthEast
            } else {
                PipeKind::SouthEast
            }
        } else {
            PipeKind::Vertical
        };
        map[start.0 as usize][start.1 as usize] = Pipe::new(start_kind, start);
        Pipes { map, start }
    }

    fn find_path(&self) -> i64 {
        let mut been = HashSet::new();
        been.insert(self.start);
        let (steps, _) = self.find_path_rec(self.start, 0, been);
        steps / 2
    }

    fn find_path_rec(&self, current_position: (i64, i64), steps: i64, been: HashSet<(i64, i64)>) -> (i64, HashSet<(i64, i64)>) {
        let pipe = &self.map[current_position.0 as usize][current_position.1 as usize];
        let possible_next_points = vec![
            (current_position.0, current_position.1 - 1),
            (current_position.0, current_position.1 + 1),
            (current_position.0 - 1, current_position.1),
            (current_position.0 + 1, current_position.1),
        ];
        for possible_next_point in possible_next_points.iter() {
            if been.contains(possible_next_point) {
                continue;
            }
            if pipe.connects_to_position(*possible_next_point) {
                let mut been = been.clone();
                been.insert(*possible_next_point);
                return self.find_path_rec(*possible_next_point, steps + 1, been);
            }
        }
        (steps + 1, been)
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let pipes = Pipes::parse(&input);
    let farthest_point = pipes.find_path();
    println!("Farthest point: {}", farthest_point); // 6725
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connections() {
        assert_eq!(true, Pipe::new(PipeKind::Vertical, (0, 0)).connects_to_position((1, 0)));
        assert_eq!(true, Pipe::new(PipeKind::Vertical, (1, 0)).connects_to_position((0, 0)));

        assert_eq!(true, Pipe::new(PipeKind::Horizontal, (0, 0)).connects_to_position((0, 1)));
        assert_eq!(true, Pipe::new(PipeKind::Horizontal, (0, 1)).connects_to_position((0, 0)));

        assert_eq!(true, Pipe::new(PipeKind::NorthEast, (1, 1)).connects_to_position((0, 1)));
        assert_eq!(true, Pipe::new(PipeKind::NorthEast, (1, 1)).connects_to_position((1, 2)));

        assert_eq!(true, Pipe::new(PipeKind::NorthWest, (1, 1)).connects_to_position((0, 1)));
        assert_eq!(true, Pipe::new(PipeKind::NorthWest, (1, 1)).connects_to_position((1, 0)));

        assert_eq!(true, Pipe::new(PipeKind::SouthWest, (1, 1)).connects_to_position((2, 1)));
        assert_eq!(true, Pipe::new(PipeKind::SouthWest, (1, 1)).connects_to_position((1, 0)));

        assert_eq!(true, Pipe::new(PipeKind::SouthEast, (1, 1)).connects_to_position((2, 1)));
        assert_eq!(true, Pipe::new(PipeKind::SouthEast, (1, 1)).connects_to_position((1, 2)));
    }

    #[test]
    fn test_simple_loop() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let pipes = Pipes::parse(input);
        assert_eq!(PipeKind::NorthEast, pipes.map[0][1].kind);
        assert_eq!(PipeKind::SouthEast, pipes.map[4][4].kind);
        assert_eq!(pipes.find_path(), 4);
    }

    #[test]
    fn test_complex_loop() {
        let input = "7-F7-
.FJ|7
FSLL7
|F--J
LJ.LJ";
        let pipes = Pipes::parse(input);
        assert_eq!(pipes.find_path(), 8);
    }
}
