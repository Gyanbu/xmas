use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer = 0;
    let mut dial = 50;
    for instrution in input {
        let step = if *instrution > 0 { 1 } else { -1 };
        for _ in 0..instrution.abs() {
            dial = (dial + step) % 100;
            if dial == 0 {
                answer += 1;
            }
        }
    }
    answer.to_string()
}
