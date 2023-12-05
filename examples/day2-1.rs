use advent_of_code::day_2::part_one;

fn main() {
    const INPUT: &str = include_str!("day2-input.txt");
    let sum = part_one::sum_ids(INPUT);
    println!("Sum of ids: {sum}");
}
