use std::ops::RangeInclusive;

pub type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);

pub fn parse(input: String) -> Input {
    let mut input = input.lines();

    let mut ranges = Vec::new();
    for line in input.by_ref() {
        if line.is_empty() {
            break;
        }
        let range: Vec<usize> = line.split('-').map(|num| num.parse().unwrap()).collect();
        ranges.push(range[0]..=range[1]);
    }

    let mut ingredients: Vec<usize> = Vec::new();
    for line in input {
        ingredients.push(line.parse().unwrap());
    }
    (ranges, ingredients)
}
