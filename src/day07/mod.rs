use std::fmt::Display;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests;


#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub(crate) trait Card:
    Sized + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + From<char> + Copy + Clone + Default + Display 
{
    fn hand_type(cards: &[Self;5]) -> HandType;
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Hand<C>
where C: Card {
    pub(crate) cards: [C; 5],
    pub(crate) hand_type: HandType,
}
impl<C> Hand<C>
where
    C: Card
{
    pub fn new(cards: [C; 5]) -> Self {
        Hand {
            cards,
            hand_type: C::hand_type(&cards)
        }
    }
}
impl<C> From<&str> for Hand<C>
where
    C: Card,
{
    fn from(value: &str) -> Self {
        if value.len() != 5 {
            panic!("Invalid input of {}.  Must be a five-character string", value);
        }

        let mut cards: [C; 5] = Default::default();
        for (index, c) in value.chars().take(5).enumerate() {
            cards[index] = C::from(c);
        }
        Hand::new(cards)
    }
}
impl<C> PartialOrd for Hand<C>
where
    C: Card,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<C> Ord for Hand<C>
where
    C: Card,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => compare(&self.cards, &other.cards),
            ord => ord,
        }
    }
}
impl<C> Display for Hand<C>
where
    C: Card,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{} ({:?})",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.hand_type
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct HandBid<C>
where
    C: Card,
{
    pub(crate) hand: Hand<C>,
    pub(crate) bid: u64,
}
impl<C> From<&str> for HandBid<C>
where
    C: Card,
{
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ');
        HandBid {
            hand: Hand::from(iter.next().unwrap()),
            bid: iter.next().unwrap().parse().unwrap(),
        }
    }
}

pub(crate) fn compare<C>(a: &[C], b: &[C]) -> std::cmp::Ordering
where
    C: Card,
{
    a.iter()
        .zip(b)
        .map(|(x, y)| x.cmp(y))
        .find(|&ord| ord != std::cmp::Ordering::Equal)
        .unwrap_or(std::cmp::Ordering::Equal)
}

pub use part1::part1;
pub use part2::part2;
