use nalgebra::{Point2, Vector2};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn opposite(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::South,
            Orientation::South => Orientation::North,
            Orientation::East => Orientation::West,
            Orientation::West => Orientation::East,
        }
    }

    fn to_vec(&self) -> Vector2<i32> {
        match self {
            Orientation::North => Vector2::new(0, -1),
            Orientation::South => Vector2::new(0, 1),
            Orientation::East => Vector2::new(1, 0),
            Orientation::West => Vector2::new(-1, 0),
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: i32,
    size: Point2<i32>,
    data: Vec<char>,
}

impl Tile {
    fn from_string(s: &str) -> Tile {
        let lines = s.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
        let id = lines[0][5..9].parse::<i32>().unwrap();
        let width = lines[1].len() as i32;
        let height = (lines.len() - 1) as i32;
        Tile {
            id,
            size: Point2::new(width, height),
            data: lines[1..]
                .iter()
                .flat_map(|line| line.chars())
                .collect::<Vec<char>>(),
        }
    }

    fn count(&self, c: char) -> u32 {
        self.data
            .iter()
            .fold(0, |acc, x| if *x == c { acc + 1 } else { acc })
    }

    fn match_pattern(&mut self, pattern: &Tile) -> bool {
        let x_min = 0;
        let y_min = 0;
        let x_max = self.size[0] - pattern.size[0];
        let y_max = self.size[1] - pattern.size[1];

        let mut any_matched = false;

        for y in y_min..y_max {
            for x in x_min..x_max {
                let mut found = true;
                'outer: for yp in 0..pattern.size[1] {
                    for xp in 0..pattern.size[0] {
                        let to_match = pattern.at(Point2::new(xp, yp));
                        let actual = self.at(Point2::new(x + xp, y + yp));
                        if to_match != ' ' && actual != to_match {
                            found = false;
                            break 'outer;
                        }
                    }
                }
                if found {
                    any_matched = true;
                    for yp in 0..pattern.size[1] {
                        for xp in 0..pattern.size[0] {
                            let to_match = pattern.at(Point2::new(xp, yp));
                            if to_match != ' ' {
                                self.data[((y + yp) * self.size[1] + (x + xp)) as usize] = 'O';
                            }
                        }
                    }
                }
            }
        }
        any_matched
    }

    fn roate_and_flip_while_trying_to_match_pattern(&mut self, pattern: &Tile) {
        for _ in 0..5 {
            if self.match_pattern(pattern) {
                return;
            }
            self.rot_right();
        }
        self.flip_x();
        for _ in 0..5 {
            if self.match_pattern(pattern) {
                return;
            }
            self.rot_right();
        }
        self.flip_y();
        for _ in 0..5 {
            if self.match_pattern(pattern) {
                return;
            }
            self.rot_right();
        }
        self.flip_x();
        for _ in 0..5 {
            if self.match_pattern(pattern) {
                return;
            }
            self.rot_right();
        }
        panic!("could not match pattern");
    }

    fn is_inside(&self, p: Point2<i32>) -> bool {
        p[0] >= 0 && p[0] < self.size[0] && p[1] >= 0 && p[1] < self.size[1]
    }

    fn at(&self, p: Point2<i32>) -> char {
        self.data[(p[1] * self.size[0] + p[0]) as usize]
    }

    fn flip_x(&mut self) {
        let mut ret = vec![' '; self.data.len()];
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                ret[(y * self.size[0] + x) as usize] =
                    self.data[(y * self.size[0] + (self.size[0] - 1 - x)) as usize];
            }
        }
        self.data = ret;
    }

    fn flip_y(&mut self) {
        let mut ret = vec![' '; self.data.len()];
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                ret[(y * self.size[0] + x) as usize] =
                    self.data[((self.size[1] - 1 - y) * self.size[0] + x) as usize];
            }
        }
        self.data = ret;
    }

    fn rot_right(&mut self) {
        let mut ret = vec![' '; self.data.len()];
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                ret[(y * self.size[0] + x) as usize] =
                    self.data[((self.size[1] - 1 - x) * self.size[0] + y) as usize];
            }
        }
        self.data = ret;
    }

    // fn rot_left(&mut self) {
    //     let mut ret = vec![' '; self.data.len()];
    //     for y in 0..self.size[1] {
    //         for x in 0..self.size[0] {
    //             ret[(y * self.size[0] + x) as usize] =
    //                 self.data[(x * self.size[0] + (self.size[1] - 1 - y)) as usize];
    //         }
    //     }
    //     self.data = ret;
    // }

    fn border_id(&self, start: Point2<i32>, dir: Vector2<i32>) -> u32 {
        let mut ret = 0u32;
        let mut p = start;
        while self.is_inside(p) {
            ret <<= 1;
            if self.at(p) == '#' {
                ret += 1;
            }
            p += dir;
        }

        ret
    }

    fn border_id_at(&self, o: Orientation) -> u32 {
        let w = self.size[0] - 1;
        let h = self.size[1] - 1;
        match o {
            Orientation::North => self.border_id(Point2::new(0, 0), Vector2::new(1, 0)),
            Orientation::East => self.border_id(Point2::new(w, 0), Vector2::new(0, 1)),
            Orientation::South => self.border_id(Point2::new(0, h), Vector2::new(1, 0)),
            Orientation::West => self.border_id(Point2::new(0, 0), Vector2::new(0, 1)),
        }
    }

    fn possible_borders(&self) -> HashSet<u32> {
        let mut ret = HashSet::new();

        let w = self.size[0] - 1;
        let h = self.size[1] - 1;

        // Clockwise
        ret.insert(self.border_id(Point2::new(0, 0), Vector2::new(1, 0)));
        ret.insert(self.border_id(Point2::new(w, 0), Vector2::new(0, 1)));
        ret.insert(self.border_id(Point2::new(w, h), Vector2::new(-1, 0)));
        ret.insert(self.border_id(Point2::new(0, h), Vector2::new(0, -1)));

        // Counterclockwise
        ret.insert(self.border_id(Point2::new(0, 0), Vector2::new(0, 1)));
        ret.insert(self.border_id(Point2::new(0, h), Vector2::new(1, 0)));
        ret.insert(self.border_id(Point2::new(w, h), Vector2::new(0, -1)));
        ret.insert(self.border_id(Point2::new(w, 0), Vector2::new(-1, 0)));

        ret
    }

    fn can_connect_to(&self, other: &Tile) -> bool {
        !self
            .possible_borders()
            .intersection(&other.possible_borders())
            .cloned()
            .collect::<HashSet<u32>>()
            .is_empty()
    }

    fn rotate_and_flip_until_it_matches(
        &mut self,
        desired_orientation: Vec<Orientation>,
        matching_candidates: &HashSet<u32>,
    ) {
        let test = |t: &Tile| {
            desired_orientation
                .iter()
                .map(|o| t.border_id_at(*o))
                .collect::<HashSet<u32>>()
                .intersection(matching_candidates)
                .cloned()
                .collect::<HashSet<u32>>()
                .len()
                == desired_orientation.len()
        };

        for _ in 0..5 {
            if test(self) {
                return;
            }
            self.rot_right();
        }
        self.flip_x();
        for _ in 0..5 {
            if test(self) {
                return;
            }
            self.rot_right();
        }
        self.flip_y();
        for _ in 0..5 {
            if test(self) {
                return;
            }
            self.rot_right();
        }
        self.flip_x();
        for _ in 0..5 {
            if test(self) {
                return;
            }
            self.rot_right();
        }
        panic!("could not find matching rotation");
    }

    #[allow(dead_code)]
    fn display(&self) {
        println!("Tile: {}", self.id);
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                print!("{}", self.at(Point2::new(x, y)));
            }
            print!("\n");
        }
    }
}

struct Image {
    tiles: HashMap<i32, Tile>,
}

impl Image {
    fn from_file(filename: &str) -> Image {
        let groups = common::read_grouped_file(filename);
        let tiles = groups
            .iter()
            .map(|s| Tile::from_string(s))
            .collect::<Vec<Tile>>();
        Image {
            tiles: HashMap::from_iter(tiles.iter().map(|t| (t.id, t.clone()))),
        }
    }

    #[allow(dead_code)]
    fn display_borders(&self) {
        for (k, v) in &self.tiles {
            println!("{} - {}", k, v.id);
            println!("{:?}", v.possible_borders());
            for (ko, vo) in &self.tiles {
                if ko == k || !v.can_connect_to(vo) {
                    continue;
                }

                println!(
                    "\t{} -> {:?}",
                    ko,
                    v.possible_borders().intersection(&vo.possible_borders())
                );
            }
            println!("\n");
        }
    }

    fn egde_border_ids(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        for (k, v) in &self.tiles {
            let mut match_cnt = 0;
            for (ko, vo) in &self.tiles {
                if ko == k {
                    continue;
                }

                if v.can_connect_to(vo) {
                    match_cnt += 1;
                }
            }
            if match_cnt == 2 {
                ret.push(v.id);
            }
        }

        ret
    }

    fn build_unified(&self) -> Tile {
        let mut candidates = self.tiles.clone();
        // Find top left corner
        let mut corner_tile = candidates
            .get(self.egde_border_ids().first().unwrap())
            .unwrap()
            .clone();
        candidates.remove(&corner_tile.id);
        let possible_matches = candidates
            .iter()
            .flat_map(|(_, t)| t.possible_borders())
            .collect::<HashSet<u32>>();
        corner_tile.rotate_and_flip_until_it_matches(
            vec![Orientation::East, Orientation::South],
            &possible_matches,
        );

        let mut finished_img = HashMap::<Point2<i32>, Tile>::new();
        finished_img.insert(Point2::new(0, 0), corner_tile.clone());
        //println!("Starting tile:");
        //corner_tile.display();
        let mut open_borders = vec![
            (
                Point2::new(0, 0),
                Orientation::South,
                corner_tile.border_id_at(Orientation::South),
            ),
            (
                Point2::new(0, 0),
                Orientation::East,
                corner_tile.border_id_at(Orientation::East),
            ),
        ];

        let mut max_x = 0;
        let mut max_y = 0;

        while !open_borders.is_empty() {
            let to_match = open_borders.first().unwrap().clone();
            let mut matched_key: Option<i32> = None;
            //println!("Trying to match ({}, {}) -> {:?} - {}:", to_match.0[0], to_match.0[1], to_match.1, to_match.2);
            for (k, v) in &candidates {
                if v.possible_borders().contains(&to_match.2) {
                    //println!("Found candidate:");
                    //println!("{:?}", v.possible_borders());
                    let mut new_tile = v.clone();
                    new_tile.rotate_and_flip_until_it_matches(
                        vec![to_match.1.opposite()],
                        &HashSet::from_iter(vec![to_match.2].iter().cloned()),
                    );
                    matched_key = Some(*k);
                    let current_pos = to_match.0 + to_match.1.to_vec();
                    max_x = std::cmp::max(max_x, current_pos[0]);
                    max_y = std::cmp::max(max_y, current_pos[1]);
                    finished_img.insert(current_pos, new_tile.clone());
                    //println!("Inserted at ({}, {})", current_pos[0], current_pos[1]);
                    //new_tile.display();
                    open_borders.remove(0);
                    open_borders.push((
                        current_pos,
                        Orientation::North,
                        new_tile.border_id_at(Orientation::North),
                    ));
                    open_borders.push((
                        current_pos,
                        Orientation::South,
                        new_tile.border_id_at(Orientation::South),
                    ));
                    open_borders.push((
                        current_pos,
                        Orientation::East,
                        new_tile.border_id_at(Orientation::East),
                    ));
                    open_borders.push((
                        current_pos,
                        Orientation::West,
                        new_tile.border_id_at(Orientation::West),
                    ));
                    break;
                }
            }

            match matched_key {
                Some(k) => {
                    candidates.remove(&k);
                }
                None => {
                    //println!("Found no match");
                    open_borders.remove(0);
                }
            }
        }

        let w = corner_tile.size[0] - 2;
        let h = corner_tile.size[1] - 2;
        let total_w = (max_x + 1) * w;
        let total_h = (max_y + 1) * h;

        let mut data = Vec::<char>::new();

        for y in 0..total_h {
            for x in 0..total_w {
                let block_x = x / w;
                let block_y = y / h;
                let local_x = (x) % w + 1;
                let local_y = (y) % h + 1;
                //println!("({}, {}) -> ({}, {}), ({}, {})", x, y, block_x, block_y, local_x, local_y);
                data.push(
                    finished_img
                        .get(&Point2::new(block_x, block_y))
                        .unwrap()
                        .at(Point2::new(local_x, local_y)),
                );
            }
        }

        Tile {
            id: 0,
            size: Point2::new(total_w, total_h),
            data,
        }
    }
}

fn main() {
    let img = Image::from_file("src/day20/input.txt");

    let border_ids = img.egde_border_ids();
    println!(
        "Solution 1: {:?} -> {}",
        border_ids,
        border_ids.iter().fold(1u128, |acc, x| acc * (*x as u128))
    );

    let monster_pattern = Tile {
        id: 0,
        size: Point2::new(20, 3),
        data: "                  # #    ##    ##    ### #  #  #  #  #  #   "
            .chars()
            .collect::<Vec<char>>(),
    };

    let mut t = img.build_unified();
    t.roate_and_flip_while_trying_to_match_pattern(&monster_pattern);
    t.display();
    println!("Solution 2: {}", t.count('#'));
}
