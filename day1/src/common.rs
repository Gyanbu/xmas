pub type Input = Vec<i32>;

pub fn parse(input: String) -> Input {
    input
        .lines()
        .map(|line| {
            let (direction, clicks) = line.split_at(1);
            match direction {
                "L" => -clicks.parse::<i32>().unwrap(),
                "R" => clicks.parse::<i32>().unwrap(),
                _ => panic!(),
            }
        })
        .collect()
}
