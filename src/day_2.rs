use std::cmp::max;

#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    pub fn create(id: u32, sets: Vec<CubeSet>) -> Game {
        Game { id, sets }
    }

    pub fn parse(line: &str) -> Option<Game> {
        let debug = line.split(':').collect::<Vec<&str>>();
        let id = debug.first()?.split(' ').collect::<Vec<&str>>();
        let id = id.last()?.parse::<u32>().ok()?;

        let sets = debug.last()?.split(';').collect::<Vec<&str>>();
        let sets: Vec<CubeSet> = sets.iter().filter_map(|str| CubeSet::parse(str)).collect();
        Some(Game { id, sets })
    }

    fn minimum_cubeset(&self) -> CubeSet {
        self.sets.iter().fold(CubeSet::create(0, 0, 0), |acc, set| {
            CubeSet::create(
                max(acc.red, set.red),
                max(acc.green, set.green),
                max(acc.blue, set.blue),
            )
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    pub fn create(red: u32, green: u32, blue: u32) -> CubeSet {
        CubeSet { red, green, blue }
    }

    fn parse(substr: &str) -> Option<CubeSet> {
        let mut set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for color_part in substr.split(',') {
            let splits = color_part
                .trim()
                .split(' ')
                .map(|str| str.trim())
                .collect::<Vec<&str>>();
            let color = splits.last()?;
            let number = splits.first()?.parse::<u32>().ok()?;
            match *color {
                "red" => set.red = number,
                "green" => set.green = number,
                "blue" => set.blue = number,
                _ => {}
            }
        }
        Some(set)
    }

    fn power(self: &CubeSet) -> u32 {
        std::cmp::max(self.red, 1) * std::cmp::max(self.green, 1) * std::cmp::max(self.blue, 1)
    }
}

pub mod part_one {
    use super::Game;
    pub fn sum_ids(input: &str) -> u32 {
        input
            .lines()
            .filter_map(Game::parse)
            .filter(|game| {
                game.sets
                    .iter()
                    .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
            })
            .map(|game| game.id)
            .sum()
    }
}

pub mod part_two {
    use super::Game;
    pub fn sum_power(input: &str) -> u32 {
        input
            .lines()
            .filter_map(Game::parse)
            .map(|game| game.minimum_cubeset())
            .map(|cubeset| cubeset.power())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::{CubeSet, Game};
    #[test]
    fn test_parse_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game::create(
            1,
            vec![
                CubeSet::create(4, 0, 3),
                CubeSet::create(1, 2, 6),
                CubeSet::create(0, 2, 0),
            ],
        );
        assert_eq!(Game::parse(line), Some(expected));
    }
}
