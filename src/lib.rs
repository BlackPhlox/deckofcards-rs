extern crate rand;

mod suit;
mod rank;
mod card;

/// Makes a card from its short string description, e.g. card!("AS") makes the Ace of Spades.
#[macro_export]
macro_rules! card {
    ($s:expr) => {
        {
            let cr = $crate::Card::from_str($s);
            if cr.is_err() {
                panic!("Not a known card {}", $s);
            }
            cr.unwrap()
        }
    };
}

#[macro_export]
macro_rules! hand {
    () => {
        Hand::new()
    };
    ( $( $s:expr ),* ) => {
        {
            let mut hand = $crate::Hand::new();
            $(
                hand.push(card!($s));
            )*
            hand
        }
    };
}

mod deck;
mod hand;

#[cfg(test)]
mod tests;

use rand::Rng;

pub use suit::Suit;
pub use rank::Rank;
pub use card::Card;
pub use deck::Deck;
pub use hand::Hand;

/// Shuffles the slice of cards
fn shuffle(cards: &mut [Card]) {
    let mut rng = rand::thread_rng();
    // Knuth shuffle
    let num_cards = cards.len();
    for i in (1 .. num_cards - 1).rev() {
        let j = rng.gen_range(0, i);
        cards.swap(i, j);
    }
}

/// Sorts the slice by suit then rank (low to high)
fn sort_suit_ascending_rank(cards: &mut [Card]) {
    cards.sort_by(|a, b| a.cmp_suit_then_rank(b));
}

/// Sorts the slice by rank(high to low) and then suit
fn sort_suit_descending_rank(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    cards.sort_by(|a, b| a.cmp_suit_then_desc_rank(b));
}

/// Sorts the slice by rank(high to low) and then suit
fn sort_descending_rank_suit(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    cards.sort_by(|a, b| a.cmp_desc_rank_then_suit(b));
}

/// Certain actions are common to a deck and a hand of cards
pub trait Cards {
    fn cards(&self) -> &[Card];
    fn mut_cards(&mut self) -> &mut [Card];

    /// Shuffle the cards into a random order
    fn shuffle(&mut self) {
        shuffle(self.mut_cards());
    }

    /// Sort the cards by suit and then by rank (low to high)
    fn sort_suit_ascending_rank(&mut self) {
        sort_suit_ascending_rank(self.mut_cards());
    }

    /// Sorts the cards by suit and then by rank (high to low)
    fn sort_suit_descending_rank(&mut self) {
        sort_suit_descending_rank(self.mut_cards());
    }

    /// Sort the cards by rank (high to low) and then by suit
    fn sort_descending_rank_suit(&mut self) {
        sort_descending_rank_suit(self.mut_cards());
    }
}
