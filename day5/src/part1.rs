use crate::common::Input;

pub fn solve(input: &Input) -> String {
    let (ranges, ingredients) = input;

    let mut answer: u128 = 0;
    for ingredient in ingredients {
        for range in ranges {
            if range.contains(ingredient) {
                answer += 1;
                break;
            }
        }
    }
    answer.to_string()
}
