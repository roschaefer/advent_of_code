use advent_of_code::day_2::parse;

fn main() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    if let Some(parsed) = parse(line) {
        println!("{:?}", parsed);
    };
}
