use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer: u128 = 0;
    for bank in input {
        let mut left_battery = (0, 0);
        for (i, battery) in bank[..bank.len() - 1].iter().enumerate() {
            if *battery > left_battery.1 {
                left_battery = (i, *battery);
            }
        }

        let mut right_battery = 0;
        for battery in &bank[left_battery.0 + 1..] {
            if *battery > right_battery {
                right_battery = *battery;
            }
        }

        let joltage = left_battery.1 * 10 + right_battery;
        answer += joltage as u128;
    }
    answer.to_string()
}
