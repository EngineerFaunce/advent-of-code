use regex::Regex;

advent_of_code::solution!(2);

struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

struct GameSet {
    cubes: Vec<Cube>,
}

struct Cube {
    amount: u32,
    color: CubeColor,
}

enum CubeColor {
    Red,
    Green,
    Blue,
}

fn parse_game_data(input: &str) -> Vec<Game> {
    let re = Regex::new(r"(\d+)").unwrap();

    input
        .lines()
        .map(|game_line| {
            let game_data: Vec<&str> = game_line.splitn(2, ':').collect();

            // 1. Parse the game id using regex which is the first number in the line
            let game_id = re
                .captures(game_data[0])
                .and_then(|cap| cap.get(0))
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or_default();

            // 2. Parse the game sets which are separated by a semi-colon
            // [3 blue, 4 red], [1 red, 2 green]
            let game_sets = game_data[1]
                .split(';')
                .map(|game_set_line| {
                    // 3. Parse the cubes which are separated by a comma.
                    // [3 blue], [4 red]
                    let game_set_data = game_set_line
                        .split(',')
                        .map(|cube_data| {
                            // 4. Parse individual cube data which is the amount followed by the color
                            // [3] [blue]
                            let cube_properties: Vec<&str> = cube_data.split_whitespace().collect();
                            Cube {
                                amount: cube_properties[0].parse::<u32>().unwrap(),
                                color: match cube_properties.get(1) {
                                    Some(&"red") => CubeColor::Red,
                                    Some(&"green") => CubeColor::Green,
                                    Some(&"blue") => CubeColor::Blue,
                                    _ => panic!("Unknown color"),
                                },
                            }
                        })
                        .collect();
                    GameSet {
                        cubes: game_set_data,
                    }
                })
                .collect::<Vec<_>>();

            Game {
                id: game_id,
                sets: game_sets,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    let games = parse_game_data(input);

    // If any of the cubes in any of the sets have a value greater than the max, then the game is invalid and the id is not counted.
    let total = games
        .iter()
        .filter(|game| {
            !game.sets.iter().any(|game_set| {
                game_set.cubes.iter().any(|cube| {
                    let max = match cube.color {
                        CubeColor::Red => RED_MAX,
                        CubeColor::Green => GREEN_MAX,
                        CubeColor::Blue => BLUE_MAX,
                    };
                    cube.amount > max
                })
            })
        })
        .map(|game| game.id)
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_game_data(input);
    let mut total = 0;

    // Process each game. Find the max number for each color among each of the game sets.
    for game in games {
        let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);

        for game_set in game.sets {
            for cube in game_set.cubes {
                match cube.color {
                    CubeColor::Red => red_max = std::cmp::max(red_max, cube.amount),
                    CubeColor::Green => green_max = std::cmp::max(green_max, cube.amount),
                    CubeColor::Blue => blue_max = std::cmp::max(blue_max, cube.amount),
                }
            }
        }
        total += red_max * green_max * blue_max;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
