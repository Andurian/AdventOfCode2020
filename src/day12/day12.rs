use std::fmt;

extern crate image;
extern crate imageproc;
extern crate nalgebra as na;

use image::{GrayImage, Luma};
use imageproc::drawing;
use na::{Point2, Vector2};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const CLOCKWISE_ORDER: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    const COUNTERCLOCKWISE_ORDER: [Direction; 4] = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    fn from_char(c: char) -> Option<Direction> {
        match c {
            x if x == 'N' => Some(Direction::North),
            x if x == 'E' => Some(Direction::East),
            x if x == 'S' => Some(Direction::South),
            x if x == 'W' => Some(Direction::West),
            _ => None,
        }
    }

    fn to_vec(&self, value: i32) -> Vector2<i32> {
        match self {
            Direction::North => Vector2::new(0, value),
            Direction::East => Vector2::new(value, 0),
            Direction::South => Vector2::new(0, -value),
            Direction::West => Vector2::new(-value, 0),
        }
    }

    fn rotate_impl(arr: &[Direction], d: Direction, amount: i32) -> Direction {
        let start = arr.iter().position(|x| *x == d).unwrap() as i32;
        let pos = (start + (amount / 90)) % 4;
        arr[pos as usize]
    }

    fn rotate(&self, turn: Turn, value: i32) -> Direction {
        match turn {
            Turn::Forward => *self,
            Turn::Left => Direction::rotate_impl(&Direction::COUNTERCLOCKWISE_ORDER, *self, value),
            Turn::Right => Direction::rotate_impl(&Direction::CLOCKWISE_ORDER, *self, value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
    Forward,
}

impl Turn {
    fn from_char(c: char) -> Option<Turn> {
        match c {
            x if x == 'L' => Some(Turn::Left),
            x if x == 'R' => Some(Turn::Right),
            x if x == 'F' => Some(Turn::Forward),
            _ => None,
        }
    }
}

struct Ship {
    orientation: Direction,
    path: Vec<Point2<i32>>,
    waypoint: Point2<i32>,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            orientation: Direction::East,
            path: vec![Point2::new(0, 0)],
            waypoint: Point2::new(10, 1),
        }
    }

    fn move_waypoint_by(&mut self, d: Direction, v: i32) {
        self.waypoint += d.to_vec(v);
    }

    fn rotate_waypoint_by(&mut self, t: Turn, mut v: i32) {
        if t == Turn::Right {
            v *= -1;
        }

        while v < 0 {
            v += 360
        }

        v %= 360;

        let p = &self.waypoint;

        match v {
            x if x == 0 => (),
            x if x == 90 => self.waypoint = Point2::new(-p[1], p[0]),
            x if x == 180 => self.waypoint = Point2::new(-p[0], -p[1]),
            x if x == 270 => self.waypoint = Point2::new(p[1], -p[0]),
            _ => panic!("Invalid rotation"),
        }
    }

    fn move_to_waypoint(&mut self, v: i32) {
        let pos = self.path.last().unwrap();
        let new_pos = pos + v * (Vector2::new(self.waypoint[0], self.waypoint[1]));
        self.path.push(new_pos);
    }

    fn move_by(&mut self, d: Direction, v: i32) {
        let new_pos = self.path.last().unwrap() + d.to_vec(v);
        self.path.push(new_pos);
    }

    fn rotate_by(&mut self, t: Turn, v: i32) {
        self.orientation = self.orientation.rotate(t, v);
    }

    fn take_action(&mut self, s: &str) {
        let instruction = s.chars().next().unwrap();
        let value = s[1..].parse::<i32>().unwrap();

        match Direction::from_char(instruction) {
            Some(dir) => {
                self.move_by(dir, value);
                return;
            }
            None => (),
        }

        match Turn::from_char(instruction) {
            Some(t) => match t {
                Turn::Forward => self.move_by(self.orientation, value),
                Turn::Left | Turn::Right => self.rotate_by(t, value),
            },
            None => panic!("Encoutered invalid action code"),
        }
    }

    fn take_waypoint_action(&mut self, s: &str) {
        let instruction = s.chars().next().unwrap();
        let value = s[1..].parse::<i32>().unwrap();

        match Direction::from_char(instruction) {
            Some(dir) => {
                self.move_waypoint_by(dir, value);
                return;
            }
            None => (),
        }

        match Turn::from_char(instruction) {
            Some(t) => match t {
                Turn::Forward => self.move_to_waypoint(value),
                Turn::Left | Turn::Right => self.rotate_waypoint_by(t, value),
            },
            None => panic!("Encoutered invalid action code"),
        }
    }

    fn travel_distance(&self) -> i32 {
        let start = self.path.first().unwrap();
        let end = self.path.last().unwrap();
        let v = end - start;

        v[0].abs() + v[1].abs()
    }

    fn draw_path(&self) -> GrayImage {
        let min_x = self.path.iter().min_by_key(|p| p[0]).unwrap()[0];
        let min_y = self.path.iter().min_by_key(|p| p[1]).unwrap()[1];
        let max_x = self.path.iter().max_by_key(|p| p[0]).unwrap()[0];
        let max_y = self.path.iter().max_by_key(|p| p[1]).unwrap()[1];

        let width = (max_x - min_x) as f32 + 20f32;
        let height = (max_y - min_y) as f32 + 20f32;

        let offset_x = -min_x + 10;
        let offset_y = -min_y + 10;

        let target_width = 1000f32;
        let target_height = 1000f32;
        let mut img = GrayImage::from_pixel(target_width as u32, target_height as u32, Luma([0]));

        self.path[..self.path.len() - 1]
            .iter()
            .zip(self.path[1..].iter())
            .for_each(|(start, end)| {
                let start_x = (start[0] + offset_x) as f32 / width * target_width;
                let end_x = (end[0] + offset_x) as f32 / width * target_width;
                let start_y = (start[1] + offset_y) as f32 / height * target_height;
                let end_y = (end[1] + offset_y) as f32 / height * target_height;
                drawing::draw_line_segment_mut(
                    &mut img,
                    (start_x, start_y),
                    (end_x, end_y),
                    Luma([255]),
                )
            });

        img
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Facing: {}\n", self.orientation).unwrap();
        write!(f, "Path taken: ").unwrap();
        self.path.iter().for_each(|&p| {
            write!(f, " {} ", p).unwrap();
        });
        write!(f, ";")
    }
}

fn calc_travel_distance(filename: &str) -> i32 {
    let instructions = common::read_file_linewise(filename);

    let mut ship = Ship::new();
    for i in instructions {
        ship.take_action(&i);
    }

    ship.draw_path().save("src/day12/path_1.png").unwrap();
    ship.travel_distance()
}

fn calc_travel_distance_by_waypoint(filename: &str) -> i32 {
    let instructions = common::read_file_linewise(filename);

    let mut ship = Ship::new();
    for i in instructions {
        ship.take_waypoint_action(&i);
    }

    ship.draw_path().save("src/day12/path_2.png").unwrap();
    ship.travel_distance()
}

fn main() {
    println!(
        "Solution 1: {}",
        calc_travel_distance("src/day12/input.txt")
    );
    println!(
        "Solution 2: {}",
        calc_travel_distance_by_waypoint("src/day12/input.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_travel_distance() {
        assert_eq!(calc_travel_distance("src/day12/input_test.txt"), 25);
    }

    #[test]
    fn test_calc_travel_distance_by_waypoint() {
        assert_eq!(
            calc_travel_distance_by_waypoint("src/day12/input_test.txt"),
            286
        );
    }
}
