use deref_derive::{Deref, DerefMut};
use std::fmt::{Display, Formatter, Result};
use std::ops::AddAssign;

use self::cards::Cards;

use super::*;

/// A `Hand` is zero or more cards that represents some aspect of a game,
/// e.g. the cards a person is holding. A hand may be shuffled or sorted
/// and there are functions for adding or removing cards. Unlike a `Deck`,
/// there is no concept of dealt or undealt cards.
#[derive(Clone, Deref, DerefMut)]
pub struct Hand(Area<Card>);

#[derive(Clone, Default)]
pub struct Area<T> {
    pub cards: Vec<T>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = String::with_capacity(self.cards.len() * 3);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            #[cfg(feature = "pretty")]
            result.push_str(&card.to_pretty());
            #[cfg(not(feature = "pretty"))]
            result.push_str(&card.to_str());
            if i < self.cards.len() - 1 {
                result.push(',');
            }
        });
        write!(f, "{}", result)
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            0: hand::Area { cards: Vec::new() },
        }
    }
}

impl<'a> AddAssign<&'a Hand> for Hand {
    fn add_assign(&mut self, rhs: &Hand) {
        self.push_hand(rhs);
    }
}

impl AddAssign<Card> for Hand {
    fn add_assign(&mut self, rhs: Card) {
        self.push_card(rhs);
    }
}

impl Cards<Card> for Hand {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }
}

impl PlayingCards for Hand {}

impl Handy<Card> for Hand {
    /// Create an empty hand
    fn new() -> Self {
        Self::default()
    }

    /// Makes a `Hand` from an existing hand
    fn from_hand(hand: &Self) -> Self {
        Self::from_cards(hand.cards())
    }

    /// Makes a `Hand` from a slice
    fn from_cards(cards: &[Card]) -> Self {
        Self {
            0: hand::Area {
                cards: Vec::from(cards),
            },
        }
    }

    /// Constructs a `Hand` from a slice of strings with abbreviated card rank / suit values
    fn from_strings(card_slice: &[&str]) -> Self {
        let cards = card_slice.iter().map(|s| card!(s)).collect::<Vec<Card>>();
        Self {
            0: hand::Area { cards },
        }
    }

    /// Adds one `Card` to the `Hand`
    fn push_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Adds zero or more cards to the `Hand`
    fn push_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    /// Adds zero or more cards from some other `Hand`
    fn push_hand(&mut self, other: &Self) {
        self.cards.extend(other.cards());
    }

    /// Returns the number of cards
    fn len(&self) -> usize {
        self.cards.len()
    }

    /// Clears the `Hand` (makes it empty)
    fn clear(&mut self) {
        self.cards.clear();
    }

    /// Removes a `Card` from the `Hand` and returns it, panics if index does not exist
    fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    /// Removes the first instance of every matching card from the `Hand`
    fn remove_cards(&mut self, cards: &[Card]) {
        for c in cards {
            let _ = self.remove_card(c);
        }
    }

    /// Removes the every instance of every matching card from the `Hand`
    fn remove_all_cards(&mut self, cards: &[Card]) {
        for c in cards {
            while self.remove_card(c) {}
        }
    }

    /// Removes first instance of the matching card from the `Hand`
    fn remove_card(&mut self, card: &Card) -> bool {
        if let Some(pos) = self.cards.iter().position(|c| c == card) {
            let _ = self.cards.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Hand {
    /// Returns cards of the specified `Rank`
    pub fn cards_of_rank(&self, rank: Rank) -> Vec<Card> {
        cards_of_rank(&self.cards, rank)
    }

    /// Returns cards of the specified `Suit`
    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        cards_of_suit(&self.cards, suit)
    }

    pub fn sort_by_suit(&self) -> Hand {
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| a.suit.partial_cmp(&b.suit).unwrap());
        Hand::from_cards(&cards)
    }

    pub fn sort_by_rank(&self) -> Hand {
        let mut cards = self.cards.clone();
        cards.sort();
        Hand::from_cards(&cards)
    }
}
