use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer = 0;
    let mut dial = 50;
    for instrution in input {
        dial = (dial + instrution) % 100;
        if dial == 0 {
            answer += 1;
        }
    }
    answer.to_string()
}
