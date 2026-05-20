# Deckard

A `no_std` compatible Rust crate for representing and manipulating standard playing cards.

## Features

- **Efficient Representation**: Cards are bit-packed into a `NonZeroU8`.
- **`no_std` Support**: Works in any environment without the standard library.
- **Rich API**: Easily manipulate ranks, suits, and card types (Normal, Wild, Custom).
- **Flexible Comparison**: Built-in support for standard card comparisons and specialized comparators like `WildCardComparator`.
- **Parsing & Display**: Support for parsing cards from strings and characters, with UTF-8 display output.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
deckard = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use deckard::{Card, Rank, Suit};

fn main() {
    // Create a card
    let card = Card::new(Rank::Ace, Suit::Spade);
    
    // Display the card
    println!("Card: {}", card); // Card: A♠
    
    // Check card properties
    assert_eq!(card.rank(), Rank::Ace);
    assert_eq!(card.suit(), Suit::Spade);
}
```

### Parsing Cards

```rust
use deckard::Card;
use core::str::FromStr;

let card = Card::from_str("A♠").unwrap(); // Ace of Spades
let heart_10 = Card::from_str("T♥").unwrap(); // Ten of Hearts
```

### Wild Cards

```rust
use deckard::{Card, CardKind, Rank, Suit};
use deckard::cmp::WildCardComparator;

let normal_card = Card::new(Rank::Ace, Suit::Spade);
let wild_card = Card::wild(Rank::King, Suit::Club);

let comparator1 = WildCardComparator::from(normal_card);
let comparator2 = WildCardComparator::from(wild_card);

assert_eq!(comparator1, comparator2);
```

### Pre-defined Decks

```rust
use deckard::Card;

let deck = Card::STANDARD_52;
assert_eq!(deck.len(), 52);
```

## License

Licensed under either of [Apache 2.0 License](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
