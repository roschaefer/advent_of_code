use ndarray::prelude::*;
use ndarray::OwnedRepr;

type Matrix = ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>;

pub mod part_one {
    use super::*;

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

    pub fn numbers_touching_symbols(matrix: Matrix) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];
        let mut current: String = "".to_string();
        let mut touched_a_number: bool = false;
        for ((x, y), value) in matrix.indexed_iter() {
            if !value.is_numeric() {
                if touched_a_number {
                    if let Ok(number) = current.parse::<u32>() {
                        result.push(number);
                    }
                };
                current = "".to_string();
                touched_a_number = false;
                continue;
            };
            current.push(*value);
            let slice = s![
                x.saturating_sub(1)..std::cmp::min(x + 2, matrix.dim().0),
                y.saturating_sub(1)..std::cmp::min(y + 2, matrix.dim().1),
            ];
            if matrix.slice(slice).iter().any(is_symbol) {
                touched_a_number = true;
            };
        }
        result
    }

    pub fn sum_part_numbers(input: &str) -> u32 {
        match parse(input) {
            Some(matrix) => numbers_touching_symbols(matrix).iter().sum(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_example() {
        assert_eq!(part_one::sum_part_numbers(INPUT), 4361);
    }
}
