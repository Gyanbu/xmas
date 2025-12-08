use std::{collections::HashSet, mem};

use crate::common::{Input, Manifold};

pub fn solve(input: &Input) -> String {
    let mut manifold = input.clone();
    manifold.simulate().to_string()
}

impl Manifold {
    fn step(&mut self) -> usize {
        let mut hit_splitters = HashSet::new();
        for mut tachyon in mem::take(&mut self.tachyons) {
            tachyon.step();
            if self.splitters.contains(&tachyon.pos) {
                hit_splitters.insert(tachyon.pos);
                for tachyon in tachyon.split() {
                    if tachyon.pos.x < self.width {
                        self.tachyons.insert(tachyon);
                    }
                }
            } else {
                self.tachyons.insert(tachyon);
            }
        }
        hit_splitters.len()
    }

    fn simulate(&mut self) -> usize {
        let mut hit_splitters = 0;
        for _ in 0..self.height - 1 {
            hit_splitters += self.step();
        }
        hit_splitters
    }
}
