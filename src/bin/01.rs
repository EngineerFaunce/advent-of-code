// use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(1);

/// Calculates the solution for part one of the Advent of Code 2023 puzzle.
/// The problem to solve involves finding the first and last digit of each line in the input string,
/// concatenating the digits together, and then summing the result.
///
/// # Arguments
///
/// * `input` - The input string containing the puzzle data.
///
/// # Returns
///
/// An optional `u32` value representing the solution, or `None` if the calculation fails.
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                // filter out all non-digit characters
                let number_chars = line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<Vec<char>>();

                // multiply the first digit by 10 and add the last digit
                (number_chars[0].to_digit(10).unwrap() * 10)
                    + number_chars[number_chars.len() - 1].to_digit(10).unwrap()
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut first_match = None;
    let mut last_match = None;
    Some(
        input
            .lines()
            .map(|line| {
                let mut matches: Vec<(usize, u32)> = Vec::new();
                for (&key, &value) in &map {
                    let indices = line.match_indices(key);
                    indices.for_each(|(pos, _)| {
                        matches.push((pos, value));
                    });
                }
                matches.sort_by(|a, b| a.0.cmp(&b.0));
                first_match = matches.first().map(|x| x.1);
                last_match = matches.last().map(|x| x.1);
                if first_match.is_none() || last_match.is_none() {
                    return 0_u32;
                }
                (first_match.unwrap() * 10) + last_match.unwrap()
            })
            .sum::<u32>(), // 54663 too high
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(323));
    }
}
