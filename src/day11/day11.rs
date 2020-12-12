extern crate nalgebra as na;
use na::{Point2, Vector2};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Floor,
    Occupied,
}

struct Neighborhood {}

impl Neighborhood {
    fn offsets() -> [Vector2<i32>; 8] {
        [
            Vector2::new(-1, -1),
            Vector2::new(0, -1),
            Vector2::new(1, -1),
            Vector2::new(-1, 0),
            Vector2::new(1, 0),
            Vector2::new(-1, 1),
            Vector2::new(0, 1),
            Vector2::new(1, 1),
        ]
    }
}

struct WaitingArea {
    area: Vec<Tile>,
    width: i32,
    height: i32,
}

impl WaitingArea {
    fn from_vec(lines: &Vec<String>) -> WaitingArea {
        WaitingArea {
            area: lines
                .iter()
                .flat_map(|line| line.chars())
                .map(|c| match c {
                    x if x == 'L' => Tile::Empty,
                    x if x == '#' => Tile::Occupied,
                    x if x == '.' => Tile::Floor,
                    _ => panic!("Unknown tile encountered"),
                })
                .collect(),
            width: lines[0].len() as i32,
            height: lines.len() as i32,
        }
    }

    fn is_inside(&self, p: &Point2<i32>) -> bool {
        p[0] >= 0 && p[0] < self.width && p[1] >= 0 && p[1] < self.height
    }

    fn at(&self, p: &Point2<i32>) -> Tile {
        if !self.is_inside(p) {
            panic!("Tried to access out of bounds element");
        }

        self.area[(p[1] * self.width + p[0]) as usize]
    }

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&Point2<i32>),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                f(&Point2::new(x, y));
            }
        }
    }

    fn count(&self, tile: Tile) -> i32 {
        let mut cnt = 0i32;
        self.for_each(|p| {
            cnt += match self.at(p) {
                x if x == tile => 1,
                _ => 0,
            }
        });
        cnt
    }

    fn step(
        &mut self,
        f_count: &dyn Fn(&WaitingArea, &Point2<i32>) -> i32,
        limits: (i32, i32),
    ) -> bool {
        let mut new_area = Vec::<Tile>::with_capacity((self.width * self.height) as usize);

        self.for_each(|p| {
            new_area.push(match self.at(&p) {
                Tile::Floor => Tile::Floor,
                Tile::Empty => {
                    if f_count(self, &p) == limits.0 {
                        Tile::Occupied
                    } else {
                        Tile::Empty
                    }
                }
                Tile::Occupied => {
                    if f_count(self, &p) >= limits.1 {
                        Tile::Empty
                    } else {
                        Tile::Occupied
                    }
                }
            });
        });

        if new_area == self.area {
            return false;
        }

        self.area = new_area;
        true
    }

    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut ret = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                ret.push(match self.at(&Point2::new(x, y)) {
                    Tile::Empty => 'L',
                    Tile::Floor => '.',
                    Tile::Occupied => '#',
                });
            }
            ret.push('\n');
        }
        ret
    }
}

fn count_occupied_neighbors(area: &WaitingArea, p: &Point2<i32>) -> i32 {
    let mut cnt = 0i32;

    for o in &Neighborhood::offsets() {
        let q = p + o;
        if !area.is_inside(&q) {
            continue;
        }
        if area.at(&q) == Tile::Occupied {
            cnt += 1;
        }
    }

    cnt
}

fn count_occupied_visible(area: &WaitingArea, p: &Point2<i32>) -> i32 {
    let mut cnt = 0i32;

    for o in &Neighborhood::offsets() {
        let mut dist = 1i32;
        let mut q = p + dist * o;
        while area.is_inside(&q) {
            match area.at(&q) {
                Tile::Occupied => {
                    cnt += 1;
                    break;
                }
                Tile::Empty => break,
                _ => (),
            }
            dist += 1;
            q = p + dist * o;
        }
    }

    cnt
}

fn stable_seats_immediate_neighborhood(filename: &str) -> i32 {
    let mut area = WaitingArea::from_vec(&common::read_file_linewise(filename));
    while area.step(&count_occupied_neighbors, (0, 4)) {}
    area.count(Tile::Occupied)
}

fn stable_seats_visible_neighborhood(filename: &str) -> i32 {
    let mut area = WaitingArea::from_vec(&common::read_file_linewise(filename));
    while area.step(&count_occupied_visible, (0, 5)) {}
    area.count(Tile::Occupied)
}

fn main() {
    println!(
        "Solution 1: {}",
        stable_seats_immediate_neighborhood("src/day11/input.txt")
    );
    println!(
        "Solution 2: {}",
        stable_seats_visible_neighborhood("src/day11/input.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seats_when_stable() {
        assert_eq!(
            stable_seats_immediate_neighborhood("src/day11/input_test.txt"),
            37
        );
    }

    #[test]
    fn test_seats_when_stable_2() {
        assert_eq!(
            stable_seats_visible_neighborhood("src/day11/input_test.txt"),
            26
        );
    }
}
