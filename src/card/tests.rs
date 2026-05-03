use core::fmt::Write;

use crate::card::{Card, CardKind, Rank, Suit};

#[test]
fn get_rank_and_suit_from_wild_card() {
    assert!(Card::WILD.is_wild());
    assert_eq!(Card::WILD.rank(), Rank::Jocker);
    assert_eq!(Card::WILD.suit(), Suit::Spade);

    let wild_card = Card::wild(Rank::King, Suit::Diamond);

    assert!(wild_card.is_wild());
    assert_eq!(wild_card.rank(), Rank::King);
    assert_eq!(wild_card.suit(), Suit::Diamond);
}

#[test]
fn card_ord_by_value() {
    assert!(Card::STANDARD_52.is_sorted());
}

#[test]
fn card_to_kind() {
    let mut card = Card::wild(Rank::Queen, Suit::Diamond);

    assert!(card.is_wild());

    card = card.to_kind(CardKind::Wild);
    assert!(card.is_wild());

    card = card.to_normal();
    assert!(!card.is_wild());
    assert_eq!(card, Card::new(Rank::Queen, Suit::Diamond));

    card = card.to_kind(CardKind::Normal);
    assert!(!card.is_wild());

    card = card.to_wild();
    assert!(card.is_wild());
    assert_eq!(card, Card::wild(Rank::Queen, Suit::Diamond));
}

#[test]
fn card_to_suit() {
    let mut card = Card::SEVEN_SPADE;

    assert_eq!(card.kind(), CardKind::Normal);
    assert_eq!(card.suit(), Suit::Spade);
    assert_eq!(card.rank(), Rank::Seven);

    card = card.to_suit(Suit::Heart);

    assert_eq!(card.kind(), CardKind::Normal);
    assert_eq!(card.suit(), Suit::Heart);
    assert_eq!(card.rank(), Rank::Seven);

    card = card.to_suit(Suit::Club);

    assert_eq!(card.kind(), CardKind::Normal);
    assert_eq!(card.suit(), Suit::Club);
    assert_eq!(card.rank(), Rank::Seven);

    card = card.to_suit(Suit::Club);
    assert_eq!(card.suit(), Suit::Club);
}

#[test]
fn card_to_rank() {
    let mut card = Card::SEVEN_SPADE;

    assert_eq!(card.kind(), CardKind::Normal);
    assert_eq!(card.suit(), Suit::Spade);
    assert_eq!(card.rank(), Rank::Seven);

    card = card.to_rank(Rank::Jocker);

    assert_eq!(card.kind(), CardKind::Normal);
    assert_eq!(card.suit(), Suit::Spade);
    assert_eq!(card.rank(), Rank::Jocker);

    card = card.to_rank(Rank::Two);

    assert_eq!(card.kind(), CardKind::Normal);
    assert_eq!(card.suit(), Suit::Spade);
    assert_eq!(card.rank(), Rank::Two);

    card = card.to_rank(Rank::Two);
    assert_eq!(card.rank(), Rank::Two);
}

#[test]
fn format_card() {
    assert_eq!(Card::ACE_SPADE.to_string().as_str(), "A♠");
    assert_eq!(Card::JACK_HEART.to_string().as_str(), "J♥");
    assert_eq!(Card::KING_DIAMOND.to_string().as_str(), "K♦");
    assert_eq!(Card::QUEEN_CLUB.to_string().as_str(), "Q♣");
    assert_eq!(Card::TEN_DIAMOND.to_string().as_str(), "T♦");
    assert_eq!(Card::TWO_HEART.to_string().as_str(), "2♥");
    assert_eq!(Card::FIVE_SPADE.to_string().as_str(), "5♠");
}

#[test]
fn parse_valid_normal_card() {
    let mut stack = Vec::with_capacity(Card::STANDARD_COUNT);

    for suit in Suit::ALL {
        for rank in Rank::STANDARD {
            let mut string = String::with_capacity(Card::UTF8_MAX_LEN);
            write!(&mut string, "{rank}{suit}").unwrap();
            stack.push(string);
        }
    }

    for (input, expected) in stack.iter().zip(Card::STANDARD_52) {
        let result = input.as_str().parse::<Card>();
        assert_eq!(result, Ok(expected));
        assert!(result.is_ok_and(Card::is_normal));
    }

    assert_eq!("U♣".parse::<Card>(), Ok(Card::JOCKER_CLUB));
    assert_eq!("U♦".parse::<Card>(), Ok(Card::JOCKER_DIAMOND));
    assert_eq!("U♥".parse::<Card>(), Ok(Card::JOCKER_HEART));
    assert_eq!("U♠".parse::<Card>(), Ok(Card::JOCKER_SPADE));
}

#[test]
fn parse_valid_wild_card() {
    let mut stack = Vec::with_capacity(Card::STANDARD_COUNT);

    for suit in Suit::ALL {
        for rank in Rank::STANDARD {
            let mut string = String::with_capacity(Card::UTF8_MAX_LEN);
            write!(&mut string, "*{rank}{suit}").unwrap();
            stack.push(string);
        }
    }

    for (input, expected) in stack.iter().zip(Card::STANDARD_52.map(Card::to_wild)) {
        let result = input.as_str().parse::<Card>();
        assert_eq!(result, Ok(expected));
        assert!(result.is_ok_and(Card::is_wild));
    }

    assert_eq!("*U♣".parse::<Card>(), Ok(Card::JOCKER_CLUB.to_wild()));
    assert_eq!("*U♦".parse::<Card>(), Ok(Card::JOCKER_DIAMOND.to_wild()));
    assert_eq!("*U♥".parse::<Card>(), Ok(Card::JOCKER_HEART.to_wild()));
    assert_eq!("*U♠".parse::<Card>(), Ok(Card::JOCKER_SPADE.to_wild()));
}
