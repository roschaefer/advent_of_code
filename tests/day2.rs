use advent_of_code::day_2::part_one;
use advent_of_code::day_2::part_two;

#[test]
fn test_part_one_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
    assert_eq!(part_one::sum_ids(input), 8)
}

#[test]
fn test_part_two_example_single_game() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        ";
    assert_eq!(part_two::sum_power(input), 48)
}

#[test]
fn test_part_two_example() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
    assert_eq!(part_two::sum_power(input), 2286)
}
