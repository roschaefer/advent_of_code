use advent_of_code::day_1::part_one::sum_of_calibration_values;

fn main() {
    const INPUT: &str = include_str!("day1-input.txt");
    let sum = sum_of_calibration_values(INPUT);
    println!("Sum: {sum}");
}
