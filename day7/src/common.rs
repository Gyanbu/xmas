use std::{collections::HashSet, hash::Hash};

pub type Input = Manifold;

pub fn parse(input: String) -> Input {
    let mut start = None;
    let mut splitters = Vec::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                'S' => start = Some(Point::new(x, y)),
                '^' => splitters.push(Point::new(x, y)),
                '.' => continue,
                _ => panic!(),
            }
        }
    }
    Manifold::new(
        start.unwrap(),
        splitters,
        input.lines().next().unwrap().len(),
        input.lines().count(),
    )
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Tachyon {
    pub pos: Point,
}

impl Tachyon {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }

    pub fn split(&self) -> Vec<Tachyon> {
        let mut buf = Vec::new();
        for x_offset in [-1, 1] {
            let Some(new_x) = self.pos.x.checked_add_signed(x_offset) else {
                continue;
            };
            buf.push(Tachyon::new(Point::new(new_x, self.pos.y)));
        }
        buf
    }

    pub fn step(&mut self) {
        self.pos.y += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Manifold {
    pub tachyons: HashSet<Tachyon>,
    pub width: usize,
    pub height: usize,
    pub splitters: Vec<Point>,
}

impl Manifold {
    pub fn new(start: Point, splitters: Vec<Point>, width: usize, height: usize) -> Self {
        let tachyon = Tachyon::new(start);
        Self {
            tachyons: HashSet::from([tachyon]),
            splitters,
            width,
            height,
        }
    }
}
