use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer: u128 = 0;
    for (col, operation) in input.operations.iter().enumerate() {
        let mut acc = input.numbers[0][col];
        for row in 1..input.numbers.len() {
            acc = operation.calc(acc, input.numbers[row][col])
        }
        answer += acc as u128;
    }
    answer.to_string()
}
