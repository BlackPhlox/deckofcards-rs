use std::result::Result;
use std::vec::Vec;

use self::cards::Cards;

use super::*;

/// The `Deck` represents a deck of zero or more cards. A default deck is 52 playing cards.
/// Internally the deck consists of two stacks consisting of dealt and undealt cards. The dealt stack
/// receives cards as they are dealt from the undealt stack.
///
/// The deck may be `reset()` to return it to its original state. A deck may be `shuffle()`'d to randomize
/// its order. Shuffling uses a Knuth shuffle.
///
/// A deck can contain more than one card with the same rank / suit combination although by default
/// it does not.
///
/// A deck cannot have more cards added or removed to it once it is created.
///
#[derive(Clone)]
pub struct Deck<C> {
    /// A deck contains zero or more cards
    pub cards: Vec<C>,
    /// Dealt cards are cards which have been dealt in calls but are still members of the deck
    /// they remain dealt until the deck is reshuffled or reset.
    pub dealt_cards: Vec<C>,
}

impl<C> Cards<C> for Deck<C> {
    fn cards(&self) -> &[C] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [C] {
        self.cards.as_mut_slice()
    }
}

impl SortCards for Deck<Card> {}

impl Decky<Card> for Deck<Card> {
    fn new() -> Deck<Card> {
        Deck::from_cards(Card::all_cards())
    }

    fn push(&mut self, cards: &[Card]) {
        self.cards.extend(cards.to_vec());
    }

    fn from_cards(cards: &[Card]) -> Deck<Card> {
        Deck {
            cards: cards.to_vec(),
            dealt_cards: Vec::with_capacity(cards.len()),
        }
    }

    fn dealt_count(&self) -> usize {
        self.dealt_cards.len()
    }

    fn dealt_cards(&self) -> &[Card] {
        self.dealt_cards.as_slice()
    }

    fn deal_one(&mut self) -> Result<Card, &'static str> {
        if let Some(card) = self.cards.pop() {
            self.dealt_cards.push(card);
            Ok(card)
        } else {
            Err("No cards left")
        }
    }

    fn reset(&mut self) {
        // Put cards back into undealt deck in reverse order
        self.cards.extend(self.dealt_cards.iter().rev());
        self.dealt_cards.clear();
    }
}
