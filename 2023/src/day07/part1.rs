use super::{Card, HandBid, HandType};
use crate::{count_distinct, AocError, DailyInput};
use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub(crate) enum Part1Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Card for Part1Card {
    fn hand_type(cards: &[Self; 5]) -> HandType {
        let card_counts = count_distinct(cards);
        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|count| *count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|count| *count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}
impl Default for Part1Card {
    fn default() -> Self {
        Self::Two
    }
}
impl From<char> for Part1Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid card {}", value),
        }
    }
}
impl Display for Part1Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "T"),
            Self::Jack => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}


pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let mut hbs: Vec<HandBid<Part1Card>> = input
        .get_input_lines()?
        .iter()
        .map(|line| HandBid::from(line.as_str()))
        .collect();

    hbs.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    let mut winnings: u64 = 0;
    for (i, hb) in hbs.iter().enumerate() {
        winnings += (i as u64 + 1) * hb.bid;
    }

    Ok(winnings.to_string())
}
