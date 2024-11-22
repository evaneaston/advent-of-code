use nom::{branch::alt, bytes::complete::tag, multi::separated_list0, IResult};

use super::model::{CubeSet, Game};
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red

pub fn count_color(input: &str) -> IResult<&str, (u32, String)> {
    let (input, count) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((tag("green"), tag("blue"), tag("red")))(input)?;
    Ok((input, (count, color.to_string())))
}

fn count(v: &[(u32, String)], c: &str) -> u32 {
    v.iter()
        .filter_map(|(count, color)| if color == c { Some(count) } else { None })
        .sum::<u32>()
}

pub fn cube_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, count_colors) = separated_list0(tag(", "), count_color)(input)?;
    Ok((
        input,
        CubeSet {
            green: count(&count_colors, "green"),
            blue: count(&count_colors, "blue"),
            red: count(&count_colors, "red"),
        },
    ))
}
pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_sets) = separated_list0(tag("; "), cube_set)(input)?;
    Ok((input, Game { id, cube_sets }))
}
