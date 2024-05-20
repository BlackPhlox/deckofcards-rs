use crate::{Cards, Deck, Handy};

pub trait Decky<C>: Cards<C>
where
    C: Clone,
{
    /// Creates an empty deck
    fn empty() -> Deck<C> {
        Deck {
            cards: vec![],
            dealt_cards: vec![],
        }
    }
    /// Creates a new `Deck` containing the standard set of 52 cards
    fn new() -> Deck<C>;
    /// Add zero or more cards to the undealt part of the deck
    fn push(&mut self, cards: &[C]);
    /// Creates a new `Deck` containing the specified cards
    fn from_cards(cards: &[C]) -> Deck<C>;
    /// Returns the number of remaining undealt cards in the `Deck`
    fn undealt_count(&self) -> usize {
        self.cards().len()
    }
    /// Returns the number of dealt cards in the `Deck`
    fn dealt_count(&self) -> usize;
    /// Returns the number of cards, dealt or undealt, within the `Deck`
    fn count(&self) -> usize {
        self.undealt_count() + self.dealt_count()
    }
    /// Returns the collection of dealt cards
    fn dealt_cards(&self) -> &[C];
    /// Tells you the top card (very next to be drawn) in the undealt deck
    /// without dealing it.
    fn top_card(&self) -> Option<C> {
        self.cards().last().map(|card| card.clone())
    }
    /// Tells you the bottom card (very last to be drawn) in the undealt deck
    /// without dealing it.
    fn bottom_card(&self) -> Option<C> {
        self.cards().first().map(|card| card.clone())
    }
    /// Deals the card from the undealt pile. If there are no cards left, the function
    /// will return an error.
    fn deal_one(&mut self) -> Result<C, &'static str>;
    /// Deals one or more card from the undealt pile and returns them as an array.
    fn deal(&mut self, numcards: usize) -> Vec<C> {
        let mut result: Vec<C> = Vec::with_capacity(numcards as usize);
        for _ in 0..numcards {
            if let Ok(card) = self.deal_one() {
                result.push(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        result
    }
    /// Deals one or more card straight to the `Hand`. Returns the number of cards dealt.
    fn deal_to_hand<H: Handy<C>>(&mut self, hand: &mut H, numcards: usize) -> usize {
        let mut dealt: usize = 0;
        for _ in 0..numcards {
            if let Ok(card) = self.deal_one() {
                dealt += 1;
                hand.push_card(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        dealt
    }
    /// Return the dealt cards back to the end of the undealt pile. Order is preserved according
    /// to the default order or the last shuffle.
    fn reset(&mut self);
    /// Resets and shuffles the deck
    fn reset_shuffle(&mut self) {
        self.reset();
        self.shuffle();
    }
}
