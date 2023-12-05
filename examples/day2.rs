use advent_of_code::day_2::sum_ids;

fn main() {
    const INPUT: &str = include_str!("day2-input.txt");
    let sum = sum_ids(INPUT);
    println!("Sum of ids: {sum}");
}
