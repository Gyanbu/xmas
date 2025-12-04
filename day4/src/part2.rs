use crate::common::Input;
use crate::part1::Diagram;

pub fn solve(input: &Input) -> String {
    let mut diagram = Diagram::new(input.clone());

    let mut previous_answer = 0;
    let mut answer: u128 = 0;
    loop {
        for x in 0..diagram.width {
            for y in 0..diagram.height {
                if diagram.get((x, y)) && diagram.count_neighbors((x, y)) < 4 {
                    diagram.remove((x, y));
                    answer += 1;
                }
            }
        }
        if previous_answer == answer {
            break;
        }
        previous_answer = answer;
    }
    answer.to_string()
}

impl Diagram {
    fn remove(&mut self, xy: (usize, usize)) {
        self.map[xy.1][xy.0] = false;
    }
}
