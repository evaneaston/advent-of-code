
use crate::common::{AocError, InputType};
use nom::bytes::complete::{tag, take};
use nom::IResult;
use nom::{branch::alt, multi::separated_list1, sequence::tuple};

#[derive(Debug)]
pub struct Valve {
    pub id: String,
    pub flow_rate: u32,
    pub tunnels: Vec<String>
}

pub(crate) fn valve_id(input: &str) -> IResult<&str, String> {
    take(2_usize)(input).map(|(remaining, parsed)| (remaining, parsed.to_string()))
}

pub(crate) fn parse_line(input: &str) -> IResult<&str, Valve> {
    tuple((
        tag("Valve "),
        valve_id,
        tag(" has flow rate="),
        nom::character::complete::u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), valve_id),
    ))(input)
    .map(|(input, parts)| (input, Valve { id: parts.1, flow_rate: parts.3, tunnels: parts.5 }))
}

pub fn get_input(input_type: InputType) -> Result<Vec<Valve>, AocError> {
    let input = input_type.get_input_as_single_string(16)?;
    let result = match separated_list1(tag("\n"), parse_line)(&input) {
        Ok((remaining, parsed)) => {
            if !remaining.is_empty() {
                panic!(
                    "Unable to parse entire input.\nremaining={}\nparsed={:?}",
                    remaining, parsed
                );
            }
            Ok(parsed)
        }
        Err(e) => panic!("Unable to parse entire {:?}", e),
    };
    result
}


#[cfg(test)]
mod tests {
    use crate::{common::InputType, day16::input::get_input};

    #[test]
    fn parse_example_input() {
        let input = get_input(InputType::Example).unwrap();

        println!("{:?}", input);

        assert_eq!(input.len(), 10);
        assert_eq!(input.iter().find(|v| v.id == "BB").unwrap().flow_rate, 13);

        assert_eq!(input.iter().find(|v| v.id == "DD").unwrap().tunnels, vec!["CC", "AA", "EE"]);
    }
}
