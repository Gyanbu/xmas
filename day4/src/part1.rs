use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let diagram = Diagram::new(input.clone());

    let mut answer: u128 = 0;
    for x in 0..diagram.width {
        for y in 0..diagram.height {
            if diagram.get((x, y)) && diagram.count_neighbors((x, y)) < 4 {
                answer += 1;
            }
        }
    }
    answer.to_string()
}

pub struct Diagram {
    pub map: Input,
    pub width: usize,
    pub height: usize,
}

impl Diagram {
    pub fn new(map: Input) -> Self {
        let width = map[0].len();
        let height = map.len();
        Self { map, width, height }
    }

    pub fn print(&self) {
        for row in &self.map {
            for roll in row {
                if *roll {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn get(&self, xy: (usize, usize)) -> bool {
        self.map[xy.1][xy.0]
    }

    pub fn count_neighbors(&self, xy: (usize, usize)) -> u8 {
        const NEIGHBORS: &[(isize, isize)] = &[
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        let mut neighbors = 0;
        for neighbor in NEIGHBORS {
            let Some(x) = xy.0.checked_add_signed(neighbor.0) else {
                continue;
            };
            if x >= self.width {
                continue;
            }
            let Some(y) = xy.1.checked_add_signed(neighbor.1) else {
                continue;
            };
            if y >= self.height {
                continue;
            }
            if self.get((x, y)) {
                neighbors += 1
            }
        }
        neighbors
    }
}
