use advent_of_code::day_1::part_one;
use advent_of_code::day_1::part_two;

#[test]
fn test_example_input() {
    const INPUT: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
    assert_eq!(part_one::sum_of_calibration_values(INPUT), 142);
}

#[test]
fn test_part_two() {
    const INPUT: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
    assert_eq!(part_two::sum_of_calibration_values(INPUT), 281);
}
