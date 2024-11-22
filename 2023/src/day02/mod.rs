use crate::{AocError, DailyInput};

use self::parse::game;

mod model;
mod parse;

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let games = lines.iter().map(|line| game(line.as_str()).unwrap().1);

    // 12 red cubes, 13 green cubes, and 14 blue

    Ok(games
        .filter_map(|game| {
            if game.max_red() > 12 || game.max_green() > 13 || game.max_blue() > 14 {
                None
            } else {
                Some(game.id)
            }
        })
        .sum::<u32>()
        .to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let games = lines.iter().map(|line| game(line.as_str()).unwrap().1);

    Ok(games
        .map(|game| game.max_red() * game.max_green() * game.max_blue())
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use log::debug;

    use crate::{
        day02::{parse::game, part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_maxes() {
        let game = game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
            .unwrap()
            .1;
        debug!("game: {:?}", game);
        assert_eq!(game.max_green(), 13);
        assert_eq!(game.max_red(), 20);
        assert_eq!(game.max_blue(), 6);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(DailyInput {
                day: 2,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "8"
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(DailyInput {
                day: 2,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "2286"
        );
    }
}
