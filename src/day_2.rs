#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn create(id: u32, rounds: Vec<Round>) -> Game {
        Game { id, rounds }
    }
}

#[derive(PartialEq, Debug)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    pub fn create(red: u32, green: u32, blue: u32) -> Round {
        Round { red, green, blue }
    }
}

fn to_round(substr: &str) -> Option<Round> {
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for color_part in substr.split(',') {
        let splits = color_part
            .trim()
            .split(" ")
            .map(|str| str.trim())
            .collect::<Vec<&str>>();
        let color = splits.last()?;
        let number = splits.first()?.parse::<u32>().ok()?;
        match color {
            &"red" => round.red = number,
            &"green" => round.green = number,
            &"blue" => round.blue = number,
            _ => {}
        }
    }
    Some(round)
}

pub fn parse(line: &str) -> Option<Game> {
    let debug = line.split(':').collect::<Vec<&str>>();
    let id = debug.first()?.split(' ').collect::<Vec<&str>>();
    let id = id.last()?.parse::<u32>().ok()?;

    let rounds = debug.last()?.split(';').collect::<Vec<&str>>();
    let rounds: Vec<Round> = rounds.iter().filter_map(|str| to_round(str)).collect();
    Some(Game { id, rounds })
}

pub fn sum_ids(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(parse)
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}
