use advent_of_code::day_3::part_two;

fn main() {
    env_logger::init();
    const INPUT: &str = include_str!("day3-input.txt");
    println!("{:?}", part_two::sum_gear_ratios(INPUT));
}
