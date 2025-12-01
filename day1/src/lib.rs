pub mod part1;
pub mod part2;

pub fn parse_input() -> Vec<i32> {
    let input = lib::read_input();
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
