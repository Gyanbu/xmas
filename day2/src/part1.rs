use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer: u128 = 0;
    for range in input {
        for i in range.0..=range.1 {
            let digits = i.ilog10() + 1;
            if digits % 2 != 0 {
                continue;
            }
            let half_multiplier = 10u64.pow(digits / 2);
            let left = i / half_multiplier;
            let right = i - left * half_multiplier;
            if left == right {
                answer += i as u128;
            }
        }
    }
    answer.to_string()
}
