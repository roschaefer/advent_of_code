use indexmap::{indexmap, IndexMap};
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::OwnedRepr;

type Matrix = ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>;

type Position = (usize, usize);
type Id = u32;
type SchematicNumber = u32;
type LookupIndex = IndexMap<Position, (Id, SchematicNumber)>;

pub fn parse(input: &str) -> Option<Matrix> {
    let input = input.trim();
    let nrows = input.lines().count();
    let first = input.lines().next()?;
    let ncols = first.chars().count();
    let dims = (nrows, ncols);

    let vec: Vec<char> = input
        .lines()
        .flat_map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();
    let result = Array::from_shape_vec(dims, vec).ok()?;
    Some(result)
}

fn is_symbol(char: &char) -> bool {
    char != &'.' && !char.is_numeric()
}

fn neighboring_numbers((x, y): Position, lookup_index: &LookupIndex) -> Vec<SchematicNumber> {
    vec![
        (x.saturating_sub(1), y.saturating_sub(1)),
        (x, y.saturating_sub(1)),
        (x.saturating_add(1), y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        // (x, y), omitted
        (x.saturating_add(1), y),
        (x.saturating_sub(1), y.saturating_add(1)),
        (x, y.saturating_add(1)),
        (x.saturating_add(1), y.saturating_add(1)),
    ]
    .into_iter()
    .unique_by(|position| *position)
    .filter_map(|position| lookup_index.get(&position))
    .unique_by(|(id, _number)| id)
    .map(|(_id, number)| *number)
    .collect()
}

pub fn create_lookup_index(matrix: &Matrix) -> LookupIndex {
    let mut result: LookupIndex = indexmap! {};
    let mut positions: Vec<(usize, usize)> = vec![];
    let mut current: String = "".to_string();
    let mut id: Id = 0;
    for (position, value) in matrix.indexed_iter() {
        if value.is_numeric() {
            current.push(*value);
            positions.push(position);
        } else {
            if let Ok(number) = current.parse::<u32>() {
                for position in positions.iter() {
                    result.insert(*position, (id, number));
                }
                id += 1;
            }
            positions = vec![];
            current = "".to_string();
        };
    }
    result.sort_keys();
    result
}

pub mod part_one {
    use super::*;

    fn part_numbers(matrix: &Matrix) -> Vec<u32> {
        let mut result: Vec<SchematicNumber> = vec![];
        let lookup_index = create_lookup_index(matrix);
        for (position, value) in matrix.indexed_iter() {
            if !is_symbol(value) {
                continue;
            };
            result.extend(neighboring_numbers(position, &lookup_index));
        }
        result
    }

    pub fn sum_part_numbers(input: &str) -> u32 {
        match parse(input) {
            Some(matrix) => part_numbers(&matrix).iter().sum(),
            None => 0,
        }
    }
}

pub mod part_two {
    use super::*;
    fn gear_ratios(matrix: &Matrix) -> Vec<(u32, u32)> {
        let mut result: Vec<(u32, u32)> = vec![];
        let lookup_index = create_lookup_index(matrix);
        for (position, value) in matrix.indexed_iter() {
            if value != &'*' {
                continue;
            };
            let neighbors = neighboring_numbers(position, &lookup_index);
            if neighbors.len() != 2 {
                continue;
            };
            let gear_numbers = (neighbors[0], neighbors[1]);
            result.push(gear_numbers);
        }
        result
    }

    pub fn sum_gear_ratios(input: &str) -> u32 {
        match parse(input) {
            Some(matrix) => gear_ratios(&matrix)
                .iter()
                .map(|(first, second)| first * second)
                .sum(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    const INPUT: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

    #[test]
    fn test_example_lookup_index() {
        let matrix = parse(INPUT).unwrap();
        let index = create_lookup_index(&matrix);
        assert_debug_snapshot!(index);
    }

    #[test]
    fn test_example_part_one() {
        assert_eq!(part_one::sum_part_numbers(INPUT), 4361);
    }

    #[test]
    fn test_example_part_two() {
        assert_eq!(part_two::sum_gear_ratios(INPUT), 467835);
    }
}
