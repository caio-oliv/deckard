use crate::{
    card::{Card, Rank, Suit},
    cmp::impls::WildCardComparator,
};

#[test]
fn wild_card_eq_all() {
    let wild_card = WildCardComparator(Card::wild(Rank::Ten, Suit::Heart));

    assert!(wild_card.is_wild());

    assert_eq!(wild_card, Card::wild(Rank::Four, Suit::Club).into());
    assert_eq!(wild_card, Card::WILD.into());

    for card in Card::STANDARD_52 {
        assert_eq!(wild_card, card.into());
    }
}

#[test]
fn wild_card_partial_ord_by_value() {
    assert!(Card::STANDARD_52.map(WildCardComparator).is_sorted());
}
