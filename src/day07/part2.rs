use std::{collections::HashMap, fmt::Display};

use crate::{AocError, DailyInput};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub(crate) enum Card {
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
impl Default for Card {
    fn default() -> Self {
        Self::Two
    }
}
impl From<char> for Card {
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
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Joker => write!(f, "J"),
            Card::Two => write!(f, "2"),
            Card::Three => write!(f, "3"),
            Card::Four => write!(f, "4"),
            Card::Five => write!(f, "5"),
            Card::Six => write!(f, "6"),
            Card::Seven => write!(f, "7"),
            Card::Eight => write!(f, "8"),
            Card::Nine => write!(f, "9"),
            Card::Ten => write!(f, "T"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut card_counts = count_distinct(cards);
        let joker_count = card_counts.remove(&Card::Joker).unwrap_or(0);

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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Hand {
    pub(crate) cards: [Card; 5],
    pub(crate) hand_type: HandType,
}
impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let hand_type = HandType::from(&cards);
        Hand { cards, hand_type }
    }
}
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        if value.len() != 5 {
            panic!("Invalid input of {}.  Must be a five-character string", value);
        }

        let mut cards: [Card; 5] = Default::default();
        for (index, c) in value.chars().take(5).enumerate() {
            cards[index] = Card::from(c);
        }
        Hand::new(cards)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => Some(compare(&self.cards, &other.cards)),
            ord => ord,
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare(&self.cards, &other.cards)
    }
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{} ({:?})",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.hand_type
        )
    }
}

pub(crate) fn compare(a: &[Card], b: &[Card]) -> std::cmp::Ordering {
    a.iter()
        .zip(b)
        .map(|(x, y)| x.cmp(y))
        .find(|&ord| ord != std::cmp::Ordering::Equal)
        .unwrap_or(std::cmp::Ordering::Equal)
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct HandBid {
    hand: Hand,
    bid: u64,
}
impl From<&str> for HandBid {
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ');
        HandBid {
            hand: Hand::from(iter.next().unwrap()),
            bid: iter.next().unwrap().parse().unwrap(),
        }
    }
}

fn count_distinct<T>(values: &[T]) -> HashMap<&T, usize>
where
    T: Eq + PartialEq + std::hash::Hash,
{
    values.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    })
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let mut hbs: Vec<HandBid> = input
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
