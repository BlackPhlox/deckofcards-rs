use std::fmt;

use deckofcards::{Area, Cardy, Deck, PlayingCards};
use deref_derive::{Deref, DerefMut};
use handy_derive::Handy;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TriCard {
    #[default]
    A,
    B,
    C,
}

#[derive(Handy, Clone, Deref, DerefMut)]
#[handy_cards(TriCard)]
pub struct Base(Area<TriCard>);

fn main() {
    let mut deck = Deck::new();

    let b = Base::new();
    //println!("{:?}", b);

    // Shuffle the deck
    deck.shuffle();

    // Deal a card
    for _ in 0..10 {
        if let Ok(card) = deck.deal_one() {
            println!("You dealt a {}", card.name());
        } else {
            panic!("We should have enough cards for this not to happen")
        }
    }

    // Put dealt cards back onto the deck
    deck.reset();
}
