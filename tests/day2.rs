use advent_of_code::day_2::{parse, Game, Round};

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
