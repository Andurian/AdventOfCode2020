use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileState {
    Black,
    White,
}

impl TileState {
    fn flipped(&self) -> TileState {
        match self {
            TileState::Black => TileState::White,
            TileState::White => TileState::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn offset(&self) -> Vector3<i32> {
        match self {
            Direction::East => Vector3::new(1, 1, 0),
            Direction::SouthEast => Vector3::new(1, 0, -1),
            Direction::SouthWest => Vector3::new(0, -1, -1),
            Direction::West => Vector3::new(-1, -1, 0),
            Direction::NorthWest => Vector3::new(-1, 0, 1),
            Direction::NorthEast => Vector3::new(0, 1, 1),
        }
    }
}

fn path_from_string(mut s: &str) -> Vec<Direction> {
    let mut ret = Vec::new();

    while !s.is_empty() {
        let mut crs = s.chars();
        let c = crs.next().unwrap();
        match c {
            x if x == 'e' => {
                ret.push(Direction::East);
                s = &s[1..];
            }
            x if x == 'w' => {
                ret.push(Direction::West);
                s = &s[1..];
            }
            x if x == 's' => {
                let c2 = crs.next().unwrap();
                match c2 {
                    y if y == 'e' => {
                        ret.push(Direction::SouthEast);
                        s = &s[2..];
                    }
                    y if y == 'w' => {
                        ret.push(Direction::SouthWest);
                        s = &s[2..];
                    }
                    _ => panic!("unknown dir"),
                }
            }
            x if x == 'n' => {
                let c2 = crs.next().unwrap();
                match c2 {
                    y if y == 'e' => {
                        ret.push(Direction::NorthEast);
                        s = &s[2..];
                    }
                    y if y == 'w' => {
                        ret.push(Direction::NorthWest);
                        s = &s[2..];
                    }
                    _ => panic!("unknown dir"),
                }
            }
            _ => panic!("unknown dir"),
        }
    }
    ret
}

fn init_tiles(filename: &str) -> HashMap<Point3<i32>, TileState> {
    let dirs = common::parse_file_linewise(filename, |line| path_from_string(line));
    let mut tile_flips = HashMap::<Point3<i32>, TileState>::new();

    for dir in &dirs {
        let p = dir
            .iter()
            .fold(Point3::<i32>::new(0, 0, 0), |acc, d| acc + d.offset());
        match tile_flips.get_mut(&p) {
            Some(v) => {
                *v = v.flipped();
            }
            None => {
                tile_flips.insert(p, TileState::Black);
            }
        }
    }

    tile_flips
}

fn solve_1(filename: &str) -> i32 {
    init_tiles(filename).iter().fold(0, |acc, (_, v)| {
        acc + if *v == TileState::Black { 1 } else { 0 }
    })
}

fn solve_2(filename: &str) -> i32 {
    let mut tiles = init_tiles(filename);

    for _ in 0..100 {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_z = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for (k, _) in &tiles {
            min_x = std::cmp::min(min_x, k[0]);
            min_y = std::cmp::min(min_y, k[1]);
            min_z = std::cmp::min(min_z, k[2]);
            max_x = std::cmp::max(max_x, k[0]);
            max_y = std::cmp::max(max_y, k[1]);
            max_z = std::cmp::max(max_z, k[2]);
        }

        let mut new_tiles = HashMap::<Point3<i32>, TileState>::new();

        for z in (min_z - 2)..(max_z + 2) {
            for y in (min_y - 2)..(max_y + 2) {
                for x in (min_x - 2)..(max_x + 2) {
                    let p = Point3::new(x, y, z);
                    let black_neighbors = vec![
                        Direction::East,
                        Direction::SouthEast,
                        Direction::SouthWest,
                        Direction::West,
                        Direction::NorthWest,
                        Direction::NorthEast,
                    ]
                    .iter()
                    .fold(0, |acc, d| match tiles.get(&(p + d.offset())) {
                        Some(v) => {
                            if *v == TileState::Black {
                                acc + 1
                            } else {
                                acc
                            }
                        }
                        None => acc,
                    });

                    let p_state = tiles.get(&p);
                    if p_state.is_some() && *p_state.unwrap() == TileState::Black{
                        if black_neighbors == 1 || black_neighbors == 2{
                            new_tiles.insert(p, TileState::Black);
                        }
                    }
                    else{
                        if black_neighbors == 2{
                            new_tiles.insert(p, TileState::Black);
                        }
                    }
                }
            }
        }

        tiles = new_tiles;

        // println!("At {}: {}",i+1, tiles.iter().fold(0, |acc, (_, v)| {
        //     acc + if *v == TileState::Black { 1 } else { 0 }
        // }));
    }
    
    tiles.iter().fold(0, |acc, (_, v)| {
        acc + if *v == TileState::Black { 1 } else { 0 }
    })
}

fn main() {
    // TODO: Might be sped up by replacing the HashMap with a HashSet of only black tiles
    println!("Solution 1: {}", solve_1("src/day24/input.txt"));
    println!("Solution 2: {}", solve_2("src/day24/input.txt"));
}
