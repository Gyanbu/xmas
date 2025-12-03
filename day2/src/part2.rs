use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut answer: u128 = 0;
    for range in input {
        for i in range.0..=range.1 {
            let digits = i.ilog10() + 1;
            let half_multiplier = 10u64.pow(digits.div_ceil(2));
            if half_multiplier % 2 != 0 {
                let left = i / half_multiplier;
                let right = i - left * half_multiplier;
                if left == right {
                    answer += i as u128;
                }
            } else {
                let mut num_buf = Vec::with_capacity(digits as usize);
                let mut exponent = 1;
                for _ in 0..digits {
                    num_buf.push((i / exponent) % 10);
                    exponent *= 10;
                }
                'outer: for window_size in 1..=digits / 2 {
                    if digits % window_size != 0 {
                        continue;
                    };
                    let pattern = &num_buf[..window_size as usize];
                    for chunk in num_buf[window_size as usize..].chunks_exact(window_size as usize)
                    {
                        if chunk != pattern {
                            continue 'outer;
                        }
                    }
                    answer += i as u128;
                    break;
                }
            }
        }
    }
    answer.to_string()
}
