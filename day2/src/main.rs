mod common;
mod part1;
mod part2;

fn main() {
    let input = utils::read_input();
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    // .to_string();
    let parsed_data = common::parse(input);

    let result1 = part1::solve(&parsed_data);
    println!("Part 1: {}", result1);

    let result2 = part2::solve(&parsed_data);
    println!("Part 2: {}", result2);
}
