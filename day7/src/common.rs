use std::collections::HashSet;

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
