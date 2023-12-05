use advent_of_code::day_2::part_two;

fn main() {
    const INPUT: &str = include_str!("day2-input.txt");
    let sum = part_two::sum_power(INPUT);
    println!("Sum of ids: {sum}");
}
