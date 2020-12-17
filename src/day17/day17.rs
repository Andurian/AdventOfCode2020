#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate itertools;

use nalgebra::{Point3, Point4, Vector3, Vector4};
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

struct PocketDimension {
    cubes: HashSet<Point3<i32>>,
}

impl PocketDimension {
    fn from_file(filename: &str) -> PocketDimension {
        let mut cubes = HashSet::<Point3<i32>>::new();
        for (i_line, line) in common::read_file_linewise(filename).iter().enumerate() {
            for (i_col, c) in line.chars().enumerate() {
                if c == '#' {
                    cubes.insert(Point3::new(i_col as i32, i_line as i32, 0));
                }
            }
        }
        PocketDimension { cubes }
    }

    fn neighbor_offsets() -> Vec<Vector3<i32>> {
        lazy_static! {
            static ref OFFSETS: Vec<Vector3<i32>> = iproduct!(0..3, 0..3, 0..3)
                .map(|(x, y, z)| Vector3::new(x - 1, y - 1, z - 1))
                .filter(|v| *v != Vector3::new(0, 0, 0))
                .collect();
        }
        OFFSETS.to_vec()
    }

    fn at(&self, p: Point3<i32>) -> CubeState {
        if self.cubes.contains(&p) {
            CubeState::Active
        } else {
            CubeState::Inactive
        }
    }

    fn bounding_box(&self) -> (Point3<i32>, Point3<i32>) {
        let mut min = Point3::<i32>::new(0, 0, 0);
        let mut max = Point3::<i32>::new(0, 0, 0);

        for k in &self.cubes {
            for i in 0..3 {
                min[i] = std::cmp::min(min[i], k[i]);
                max[i] = std::cmp::max(max[i], k[i]);
            }
        }

        (min, max)
    }

    fn active_neighbors(&self, p: Point3<i32>) -> i32 {
        PocketDimension::neighbor_offsets()
            .iter()
            .fold(0, |acc, v| {
                if self.at(p + v) == CubeState::Active {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn active_cubes(&self) -> i32 {
        self.cubes.len() as i32
    }

    fn step(&mut self) {
        let mut new_cubes = HashSet::<Point3<i32>>::new();
        let (min, max) = self.bounding_box();
        for z in min[2] - 1..max[2] + 2 {
            for y in min[1] - 1..max[1] + 2 {
                for x in min[0] - 1..max[0] + 2 {
                    let p = Point3::<i32>::new(x, y, z);
                    let n = self.active_neighbors(p);
                    let s = self.at(p);
                    if (s == CubeState::Active && (n == 2 || n == 3))
                        || (s == CubeState::Inactive && n == 3)
                    {
                        new_cubes.insert(p);
                    }
                }
            }
        }
        self.cubes = new_cubes;
    }
}

impl Debug for PocketDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.bounding_box();
        for z in min[2]..max[2] + 1 {
            f.write_fmt(format_args!("Z={}\n", z)).unwrap();
            for y in min[1]..max[1] + 1 {
                for x in min[0]..max[0] + 1 {
                    f.write_str(match self.at(Point3::<i32>::new(x, y, z)) {
                        CubeState::Active => "#",
                        CubeState::Inactive => ".",
                    })
                    .unwrap();
                }
                f.write_str("\n").unwrap();
            }
            f.write_str("\n\n").unwrap();
        }

        f.write_str("\n")
    }
}

struct PocketDimension4 {
    cubes: HashSet<Point4<i32>>,
}

impl PocketDimension4 {
    fn from_file(filename: &str) -> PocketDimension4 {
        let mut cubes = HashSet::<Point4<i32>>::new();
        for (i_line, line) in common::read_file_linewise(filename).iter().enumerate() {
            for (i_col, c) in line.chars().enumerate() {
                if c == '#' {
                    cubes.insert(Point4::new(i_col as i32, i_line as i32, 0, 0));
                }
            }
        }
        PocketDimension4 { cubes }
    }

    fn neighbor_offsets() -> Vec<Vector4<i32>> {
        lazy_static! {
            static ref OFFSETS: Vec<Vector4<i32>> = iproduct!(0..3, 0..3, 0..3, 0..3)
                .map(|(x, y, z, w)| Vector4::new(x - 1, y - 1, z - 1, w - 1))
                .filter(|v| *v != Vector4::new(0, 0, 0, 0))
                .collect();
        }
        OFFSETS.to_vec()
    }

    fn at(&self, p: Point4<i32>) -> CubeState {
        if self.cubes.contains(&p) {
            CubeState::Active
        } else {
            CubeState::Inactive
        }
    }

    fn bounding_box(&self) -> (Point4<i32>, Point4<i32>) {
        let mut min = Point4::<i32>::new(0, 0, 0, 0);
        let mut max = Point4::<i32>::new(0, 0, 0, 0);

        for k in &self.cubes {
            for i in 0..4 {
                min[i] = std::cmp::min(min[i], k[i]);
                max[i] = std::cmp::max(max[i], k[i]);
            }
        }

        (min, max)
    }

    fn active_neighbors(&self, p: Point4<i32>) -> i32 {
        PocketDimension4::neighbor_offsets()
            .iter()
            .fold(0, |acc, v| {
                if self.at(p + v) == CubeState::Active {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn active_cubes(&self) -> i32 {
        self.cubes.len() as i32
    }

    fn step(&mut self) {
        let mut new_cubes = HashSet::<Point4<i32>>::new();
        let (min, max) = self.bounding_box();
        for w in min[3] - 1..max[3] + 2 {
            for z in min[2] - 1..max[2] + 2 {
                for y in min[1] - 1..max[1] + 2 {
                    for x in min[0] - 1..max[0] + 2 {
                        let p = Point4::<i32>::new(x, y, z, w);
                        let n = self.active_neighbors(p);
                        let s = self.at(p);
                        if (s == CubeState::Active && (n == 2 || n == 3))
                            || (s == CubeState::Inactive && n == 3)
                        {
                            new_cubes.insert(p);
                        }
                    }
                }
            }
        }
        self.cubes = new_cubes;
    }
}

fn active_cubes_after_cycles(filename: &str, n: i32) -> i32 {
    let mut d = PocketDimension::from_file(filename);
    for _ in 0..n {
        d.step();
    }
    d.active_cubes()
}

fn active_hypercubes_after_cycles(filename: &str, n: i32) -> i32 {
    let mut d = PocketDimension4::from_file(filename);
    for _ in 0..n {
        d.step();
    }
    d.active_cubes()
}

fn main() {
    println!(
        "Solution1: {}",
        active_cubes_after_cycles("src/day17/input.txt", 6)
    );
    println!(
        "Solution2: {}",
        active_hypercubes_after_cycles("src/day17/input.txt", 6)
    );
}
