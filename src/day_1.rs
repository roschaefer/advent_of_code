pub mod part_one {
    fn calibration_value(line: &str) -> u32 {
        let digits: Vec<u32> = line
            .to_string()
            .chars()
            .filter_map(|char| char.to_digit(10))
            .collect();
        [digits.first(), digits.last()]
            .iter()
            .filter_map(|x| *x)
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap_or(0)
    }

    pub fn sum_of_calibration_values(document: &str) -> u32 {
        document.lines().map(calibration_value).sum()
    }
}

pub mod part_two {
    const DIGIT_MAPPING: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    fn first_digit(line: &str) -> u32 {
        let mut prefix = "".to_string();
        for char in line.chars() {
            match char.to_digit(10) {
                Some(digit) => return digit,
                None => prefix.push(char),
            }
            let prefix_slice = prefix.as_str();
            if let Some((_, number)) = DIGIT_MAPPING
                .iter()
                .find(|(word, _)| prefix_slice.ends_with(word))
            {
                return *number;
            }
        }
        0
    }

    fn last_digit(line: &str) -> u32 {
        let mut prefix = "".to_string();
        for char in line.chars().rev() {
            match char.to_digit(10) {
                Some(digit) => return digit,
                None => prefix.insert(0, char),
            }
            let prefix_slice = prefix.as_str();
            if let Some((_, number)) = DIGIT_MAPPING
                .iter()
                .find(|(word, _)| prefix_slice.starts_with(word))
            {
                return *number;
            }
        }
        0
    }

    fn calibration_value(line: &str) -> u32 {
        [first_digit(line), last_digit(line)]
            .iter()
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap_or(0)
    }

    pub fn sum_of_calibration_values(document: &str) -> u32 {
        document.lines().map(calibration_value).sum()
    }
}
