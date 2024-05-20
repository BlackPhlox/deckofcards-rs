/// Creates a `Card` and sets its rank / suit from its abbreviated string description. The description
/// is of the form "RS", Rank followed by Suit, e.g. "2D" for Two of Diamonds.
///
/// # Examples
///
/// Creates the Ace of Spades
///
/// ```
/// # #[macro_use] extern crate deckofcards;
/// # fn main() {
/// let card = card!("AS");
/// # }
/// ```
#[macro_export]
macro_rules! card {
    ($s:expr) => {{
        let cr = $crate::Card::from_str($s);
        cr.unwrap_or_else(|_| {
            panic!("Not a known card {}", $s);
        })
    }};
}

/// Creates a `Hand` of cards from the list of abbreviated cards string specified by rank / suit,
///
/// # Examples
///
/// Creates a hand containing the Queen of Hearts and Two of Diamonds.
///
/// ```
/// # #[macro_use] extern crate deckofcards;
/// # fn main() {
/// let hand = hand!("QH", "2D");
/// # }
/// ```
#[macro_export]
macro_rules! hand {
    () => {
        $crate::Hand::new()
    };
    ( $( $s:expr ),* ) => {
        {
            let mut hand = $crate::Hand::new();
            $(
                hand += card!($s);
            )*
            hand
        }
    };
}

/// Creates a new `Hand` that is the combination two hands into one hand. This does not consume
/// the original hands.
///
/// # Examples
///
/// Combine hand1 and hand2 into a new hand_combined.
///
/// ```
/// # #[macro_use] extern crate deckofcards;
/// # fn main() {
/// use deckofcards::Hand;
/// let hand1 = hand!("AS", "KD");
/// let hand2 = hand!("QH", "3C", "4S");
/// let hand_combined = combine_hands!(&hand1, &hand2);
/// # }
/// ```
#[macro_export]
macro_rules! combine_hands {
    ( $( $h: expr),* ) => {
        {
            let mut result = $crate::Hand::new();
            $(
                result += $h;
            )*
            result
        }
    }
}

/// Creates a standard deck of 52 playing cards
#[macro_export]
macro_rules! deck {
    () => {
        $crate::Deck::new()
    };
}

#[cfg(feature = "pretty")]
use colored::CustomColor;
#[cfg(feature = "pretty")]
const GRAY: CustomColor = CustomColor {
    r: 100,
    g: 100,
    b: 100,
};

mod suit;
pub use suit::{Color, Suit};

mod rank;
pub use rank::Rank;

mod joker;
pub use joker::Joker;

mod card;
pub use card::{Card, DisplayCard};

mod cards;
pub use cards::{cards_of_rank, cards_of_suit, Cards, SortCards};

mod deck;
pub use deck::Deck;

mod decky;
pub use decky::Decky;

mod hand;
pub use hand::{Area, Hand};

mod handy;
pub use handy::Handy;

#[cfg(test)]
mod tests;
