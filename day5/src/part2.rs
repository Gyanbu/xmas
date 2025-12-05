use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let mut ranges = input.0.clone();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut answer: u128 = 0;
    let mut furthest_index = 0;
    for range in ranges {
        if *range.end() <= furthest_index {
            continue;
        }
        if *range.start() <= furthest_index {
            let offset = range.end() - furthest_index;
            furthest_index = *range.end();
            answer += (range.count() - offset) as u128;
        } else {
            furthest_index = *range.end();
            answer += range.count() as u128;
        }
    }

    answer.to_string()
}
