use crate::common::{blank_line, AocError, InputType};
use log::{debug, info};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{many_m_n, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum ListItem {
    List(Vec<ListItem>),
    Number(u64),
}
impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl ListItem {
    fn list_list_compare(l: &Vec<ListItem>, r: &Vec<ListItem>) -> Ordering {
        let min_len = core::cmp::min(l.len(), r.len());
        for i in 0..min_len {
            match l[i].partial_cmp(&r[i]) {
                Some(ordering) => {
                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
                None => panic!(),
            }
        }
        if l.len() < r.len() {
            return Ordering::Less;
        } else if l.len() > r.len() {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::List(l), Self::List(r)) => Some(Self::list_list_compare(l, r)),
            (Self::Number(l), Self::Number(r)) => Some(l.cmp(r)),
            (Self::Number(l), Self::List(r)) => {
                Some(Self::list_list_compare(&vec![Self::Number(*l)], &r))
            }
            (Self::List(l), Self::Number(r)) => {
                Some(Self::list_list_compare(&l, &vec![Self::Number(*r)]))
            }
        }
    }
}

#[derive(Debug)]
struct Packet(Vec<ListItem>);

impl Clone for Packet {
    fn clone(&self) -> Self {
        Self(Vec::from_iter(self.0.iter().map(|r| r.clone())))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(blank_line, packet_pair)(input)
}

fn packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    tuple((packet, packet))(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    tuple((list, many_m_n(0, 1, tag("\n"))))(input).map(|(input, parts)| (input, Packet(parts.0)))
}

fn list(input: &str) -> IResult<&str, Vec<ListItem>> {
    delimited(tag("["), separated_list0(tag(","), list_item), tag("]"))(input)
        .map(|(input, parsed)| (input, Vec::from(parsed)))
}

fn list_item(input: &str) -> IResult<&str, ListItem> {
    alt((numeric_list_item, list_list_item))(input).map(|(input, parsed)| (input, parsed))
}

fn numeric_list_item(input: &str) -> IResult<&str, ListItem> {
    nom::character::complete::u64(input).map(|(input, parsed)| (input, ListItem::Number(parsed)))
}

fn list_list_item(input: &str) -> IResult<&str, ListItem> {
    list(input).map(|(input, parsed)| (input, ListItem::List(parsed)))
}

fn parse_packet_pairs() -> Result<Vec<(Packet, Packet)>, AocError> {
    let input = InputType::Challenge.get_input_as_single_string(13)?;
    let pairs = match packet_pairs(&input) {
        Ok(pairs) => {
            if !pairs.0.is_empty() {
                debug!("packetPairs.0={:?}", pairs.0);
                debug!("packetPairs.1={:?}", pairs.1);
                panic!("Something went wrong")
            }
            pairs.1
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    };
    Ok(pairs)
}

fn fix_ordering(pairs: &mut Vec<(Packet, Packet)>) {
    for i in 0..pairs.len() {
        let pair = &pairs[i];
        if pair.0.partial_cmp(&pair.1) == Some(Ordering::Greater) {
            let l = pair.0.clone();
            let r = pair.1.clone();

            pairs[i].0 = r;
            pairs[i].1 = l;
        }
    }
}

pub fn part1() -> Result<String, AocError> {
    let pairs = parse_packet_pairs()?;

    info!("There are {} packet pairs", pairs.len());

    let mut in_order_indices = Vec::<usize>::new();

    for i in 1..=pairs.len() {
        let pair = &pairs[i - 1];
        let comparison = pair.0.partial_cmp(&pair.1).unwrap();

        debug!("Pair #{:?}", comparison);
        if comparison == Ordering::Less {
            in_order_indices.push(i);
        }
    }

    let sum: usize = in_order_indices.iter().sum();

    assert_eq!(sum, 5350);

    Ok(format!("{}", sum))
}

pub fn part2() -> Result<String, AocError> {
    let mut pairs = parse_packet_pairs()?;
    fix_ordering(&mut pairs);

    let mut flattened_packets = Vec::<Packet>::with_capacity(pairs.len() * 2);
    for pair in pairs {
        flattened_packets.push(pair.0);
        flattened_packets.push(pair.1);
    }

    let divider1 = packet("[[2]]").unwrap().1;
    let divider2 = packet("[[6]]").unwrap().1;

    flattened_packets.push(divider1.clone());
    flattened_packets.push(divider2.clone());

    flattened_packets.sort();

    let divider1_index = flattened_packets.binary_search(&divider1).unwrap() + 1;
    let divider2_index = flattened_packets.binary_search(&divider2).unwrap() + 1;
    info!("Found {:?} at {}", divider1, divider1_index);
    info!("Found {:?} at {}", divider2, divider2_index);

    let product = divider1_index * divider2_index;

    assert_eq!(product, 19570);

    Ok(format!("{}", product))
}

#[cfg(test)]
mod tests {
    use crate::day13::{fix_ordering, packet};
    use std::cmp::Ordering;

    use super::packet_pairs;

    #[test]
    fn test_packet_compare() {
        // pair 1
        let left = packet("[1,1,3,1,1]").unwrap().1;
        let right = packet("[1,1,5,1,1]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        // pair 2
        let left = packet("[[1],[2,3,4]]").unwrap().1;
        let right = packet("[[1],4]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        // pair 3
        let left = packet("[9]").unwrap().1;
        let right = packet("[[8,7,6]]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));

        // pair 4
        let left = packet("[[4,4],4,4]").unwrap().1;
        let right = packet("[[4,4],4,4,4]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        // pair 5
        let left = packet("[7,7,7,7]").unwrap().1;
        let right = packet("[7,7,7]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));

        // pair 6
        let left = packet("[[[]]]").unwrap().1;
        let right = packet("[[]]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));

        // pair 7
        let left = packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().1;
        let right = packet("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap().1;
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));
    }

    #[test]
    fn test_ordering() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]
        
[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let (remaining, mut pairs) = packet_pairs(input).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(pairs.len(), 8);

        fix_ordering(&mut pairs);

        assert_eq!(pairs[0].0, packet("[1,1,3,1,1]").unwrap().1);
        assert_eq!(pairs[0].1, packet("[1,1,5,1,1]").unwrap().1);

        assert_eq!(pairs[1].0, packet("[[1],[2,3,4]]").unwrap().1);
        assert_eq!(pairs[1].1, packet("[[1],4]").unwrap().1);

        assert_eq!(pairs[2].0, packet("[[8,7,6]]").unwrap().1);
        assert_eq!(pairs[2].1, packet("[9]").unwrap().1);

        assert_eq!(pairs[3].0, packet("[[4,4],4,4]").unwrap().1);
        assert_eq!(pairs[3].1, packet("[[4,4],4,4,4]").unwrap().1);

        assert_eq!(pairs[4].0, packet("[7,7,7]").unwrap().1);
        assert_eq!(pairs[4].1, packet("[7,7,7,7]").unwrap().1);

        assert_eq!(pairs[5].0, packet("[]").unwrap().1);
        assert_eq!(pairs[5].1, packet("[3]").unwrap().1);

        assert_eq!(pairs[6].0, packet("[[]]").unwrap().1);
        assert_eq!(pairs[6].1, packet("[[[]]]").unwrap().1);

        assert_eq!(pairs[7].0, packet("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap().1);
        assert_eq!(pairs[7].1, packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().1);
    }
}
