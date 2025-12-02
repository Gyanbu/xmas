mod common;
mod part1;
mod part2;

fn main() {
    let input = utils::read_input();
    let parsed_data = common::parse(input);

    let result1 = part1::solve(&parsed_data);
    println!("Part 1: {}", result1);

    let result2 = part2::solve(&parsed_data);
    println!("Part 2: {}", result2);
}
