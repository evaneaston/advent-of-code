use super::{Card, HandBid, HandType};
use crate::{count_distinct, AocError, DailyInput};
use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub(crate) enum Part2Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
impl Card for Part2Card {
    fn hand_type(cards: &[Self;5]) -> HandType {
        let mut card_counts = count_distinct(cards);
        let joker_count = card_counts.remove(&Part2Card::Joker).unwrap_or(0);

        if joker_count == 5 {
            HandType::FiveOfAKind
        } else {
            match card_counts.len() {
                1 => HandType::FiveOfAKind,
                2 => {
                    if card_counts.values().any(|&count| (joker_count + count) == 4) {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    if card_counts.values().any(|&count| (joker_count + count) == 3) {
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
}
impl Default for Part2Card {
    fn default() -> Self {
        Self::Two
    }
}
impl From<char> for Part2Card {
    fn from(value: char) -> Self {
        match value {
            'J' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid card {}", value),
        }
    }
}
impl Display for Part2Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Joker => write!(f, "J"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "T"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let mut hbs: Vec<HandBid<Part2Card>> = input
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
