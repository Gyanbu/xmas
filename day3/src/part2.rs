use std::array;

use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer: u128 = 0;
    for bank in input {
        let bank_length = bank.len();
        let mut chosen_batteries: [usize; 12] = array::from_fn(|i| bank_length - 12 + i);
        let mut previous_battery: isize = -1;
        for battery in chosen_batteries.iter_mut() {
            let mut highest_left_battery: (usize, u8) = (*battery, 0);
            for (i, _) in bank
                .iter()
                .enumerate()
                .take(*battery + 1)
                .skip((previous_battery + 1) as usize)
            {
                if bank[i] > highest_left_battery.1 {
                    highest_left_battery = (i, bank[i]);
                }
            }
            *battery = highest_left_battery.0;
            previous_battery = highest_left_battery.0 as isize;
        }

        let mut jolts: u128 = 0;
        for (i, battery) in chosen_batteries.into_iter().rev().enumerate() {
            jolts += bank[battery] as u128 * 10u128.pow(i as u32);
        }
        answer += jolts;
    }
    answer.to_string()
}
