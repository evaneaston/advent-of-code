
use rand::seq::SliceRandom;

use crate::{
    day07::{
        part1::{Card, Hand, HandType, self},
        part2,
    },
    DailyInput, InputType,
};

#[test]
fn test_card_ordering() {
    assert!(Card::Three > Card::Two);
    assert!(Card::Four > Card::Three);
    assert!(Card::Five > Card::Four);
    assert!(Card::Six > Card::Five);
    assert!(Card::Seven > Card::Six);
    assert!(Card::Eight > Card::Seven);
    assert!(Card::Nine > Card::Eight);
    assert!(Card::Ten > Card::Nine);
    assert!(Card::Jack > Card::Ten);
    assert!(Card::Queen > Card::Jack);
    assert!(Card::King > Card::Queen);
    assert!(Card::Ace > Card::King);
}

#[test]
fn test_card_ranker() {
    assert!(matches!(Hand::from("32T3K").hand_type, HandType::OnePair));
    assert!(matches!(Hand::from("KK677").hand_type, HandType::TwoPair));
    assert!(matches!(Hand::from("KTJJT").hand_type, HandType::TwoPair));
    assert!(matches!(Hand::from("T55J5").hand_type, HandType::ThreeOfAKind));
    assert!(matches!(Hand::from("QQQJA").hand_type, HandType::ThreeOfAKind));
}

#[test]
fn test_hand_ranking() {
    let mut ranked = [Hand::from("32T3K"),
        Hand::from("KK677"),
        Hand::from("KTJJT"),
        Hand::from("T55J5"),
        Hand::from("QQQJA")];

    println!("Before");
    for (i, h) in ranked.iter().enumerate() {
        println!(" {i}: {}", h);
    }
    ranked.shuffle(&mut rand::thread_rng());

    println!("Shuffled");
    for (i, h) in ranked.iter().enumerate() {
        println!(" {i}: {}", h);
    }

    ranked.sort();
    println!("Sorted");
    for (i, h) in ranked.iter().enumerate() {
        println!(" {i}: {}", h);
    }

    assert!(ranked[0] < ranked[1]);
    assert!(ranked[1] < ranked[2]);
    assert!(ranked[2] < ranked[3]);
    assert!(ranked[3] < ranked[4]);
}

#[test]
fn test_part1_example() {
    assert_eq!(
        part1::part1(DailyInput {
            day: 7,
            part: None,
            input_type: InputType::Example
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
            part: None,
            input_type: InputType::Challenge
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
            part: None,
            input_type: InputType::Example
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
            part: None,
            input_type: InputType::Challenge
        })
        .unwrap(),
        "250825971"        
    );
}
