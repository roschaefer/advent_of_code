use advent_of_code::day_3::part_one;

fn main() {
    env_logger::init();
    const INPUT: &str = include_str!("day3-input.txt");
    println!("{:?}", part_one::sum_part_numbers(INPUT));
}
