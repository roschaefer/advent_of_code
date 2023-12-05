use advent_of_code::day_2::{parse, sum_ids, Game, Round};

#[test]
fn test_parse_game() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let expected = Game::create(
        1,
        vec![
            Round::create(4, 0, 3),
            Round::create(1, 2, 6),
            Round::create(0, 2, 0),
        ],
    );
    assert_eq!(parse(line), Some(expected));
}

#[test]
fn test_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
    assert_eq!(sum_ids(input), 8)
}
