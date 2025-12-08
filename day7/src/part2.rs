use std::{collections::HashMap, mem};

use crate::common::{Input, Manifold, Point, Tachyon};

pub fn solve(input: &Input) -> String {
    let manifold = input.clone();
    let mut quantum_manifold = QuantumManifold::new(manifold);
    quantum_manifold.simulate().to_string()
}

struct QuantumManifold {
    pub quantum_tachyons: HashMap<Tachyon, usize>,
    pub width: usize,
    pub height: usize,
    pub splitters: Vec<Point>,
}

impl QuantumManifold {
    fn new(manifold: Manifold) -> Self {
        let mut quantum_tachyons = HashMap::new();
        quantum_tachyons.insert(*manifold.tachyons.iter().next().unwrap(), 1);
        Self {
            quantum_tachyons,
            width: manifold.width,
            height: manifold.height,
            splitters: manifold.splitters,
        }
    }

    fn step(&mut self) {
        for (mut tachyon, ammount) in mem::take(&mut self.quantum_tachyons) {
            tachyon.step();
            if self.splitters.contains(&tachyon.pos) {
                for tachyon in tachyon.split() {
                    if tachyon.pos.x < self.width {
                        *self.quantum_tachyons.entry(tachyon).or_insert(0) += ammount;
                    }
                }
            } else {
                *self.quantum_tachyons.entry(tachyon).or_insert(0) += ammount;
            }
        }
    }

    fn simulate(&mut self) -> usize {
        for _ in 0..self.height - 1 {
            self.step();
        }
        self.quantum_tachyons.values().sum()
    }
}
