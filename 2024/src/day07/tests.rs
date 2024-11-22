use rand::seq::SliceRandom;

use crate::{
    day07::{
        part1::{self, Part1Card},
        part2, Hand, HandType,
    },
    DailyInput, InputType,
};

#[test]
fn test_card_ordering() {
    assert!(Part1Card::Three > Part1Card::Two);
    assert!(Part1Card::Four > Part1Card::Three);
    assert!(Part1Card::Five > Part1Card::Four);
    assert!(Part1Card::Six > Part1Card::Five);
    assert!(Part1Card::Seven > Part1Card::Six);
    assert!(Part1Card::Eight > Part1Card::Seven);
    assert!(Part1Card::Nine > Part1Card::Eight);
    assert!(Part1Card::Ten > Part1Card::Nine);
    assert!(Part1Card::Jack > Part1Card::Ten);
    assert!(Part1Card::Queen > Part1Card::Jack);
    assert!(Part1Card::King > Part1Card::Queen);
    assert!(Part1Card::Ace > Part1Card::King);
}

#[test]
fn test_card_ranker() {
    assert!(matches!(Hand::<Part1Card>::from("32T3K").hand_type, HandType::OnePair));
    assert!(matches!(Hand::<Part1Card>::from("KK677").hand_type, HandType::TwoPair));
    assert!(matches!(Hand::<Part1Card>::from("KTJJT").hand_type, HandType::TwoPair));
    assert!(matches!(
        Hand::<Part1Card>::from("T55J5").hand_type,
        HandType::ThreeOfAKind
    ));
    assert!(matches!(
        Hand::<Part1Card>::from("QQQJA").hand_type,
        HandType::ThreeOfAKind
    ));
}

#[test]
fn test_hand_ranking() {
    let ranked = [
        Hand::<Part1Card>::from("32T3K"),
        Hand::<Part1Card>::from("KTJJT"),
        Hand::<Part1Card>::from("KK677"),
        Hand::<Part1Card>::from("T55J5"),
        Hand::<Part1Card>::from("QQQJA"),
    ];

    for i in 0..ranked.len()-1 {
        assert!(
            ranked[i] < ranked[i + 1],
            " Failed because {} < {} evaluated to false",
            ranked[i],
            ranked[i + 1]
        );
    }

    let mut rng = rand::thread_rng();
    let mut shuffled: Vec<_> = ranked.to_vec();
    loop {
        shuffled.shuffle(&mut rng);
        if shuffled != ranked {
            break;
        }
    }

    shuffled.sort();

    for i in 0..shuffled.len()-1 {
        assert!(
            shuffled[i] < shuffled[i + 1],
            " Failed because {} < {} evaluated to false",
            shuffled[i],
            shuffled[i + 1]
        );
    }
}

#[test]
fn test_part1_example() {
    assert_eq!(
        part1::part1(DailyInput {
            day: 7,
            input_type: InputType::Example,
            number: None,
        })
        .unwrap(),
        "6440"
    );
}

#[test]
fn test_part1_challenge() {
    assert_eq!(
        part1::part1(DailyInput {
            day: 7,
            input_type: InputType::Challenge,
            number: None,
        })
        .unwrap(),
        "251216224"
    );
}

#[test]
fn test_part2_example() {
    assert_eq!(
        part2::part2(DailyInput {
            day: 7,
            input_type: InputType::Example,
            number: None,
        })
        .unwrap(),
        "5905"
    );
}

#[test]
fn test_part2_challenge() {
    assert_eq!(
        part2::part2(DailyInput {
            day: 7,
            input_type: InputType::Challenge,
            number: None,
        })
        .unwrap(),
        "250825971"
    );
}
