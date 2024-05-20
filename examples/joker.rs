use std::fmt;

use deckofcards::{Area, Card, Deck, Decky, DisplayCard, Joker};
use deref_derive::{Deref, DerefMut};
use handy_derive::Handy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CardWithJoker {
    Card(Card),
    Joker(Joker),
}

impl fmt::Display for CardWithJoker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardWithJoker::Card(c) => write!(f, "{}", c),
            CardWithJoker::Joker(j) => write!(f, "{}", j),
        }
    }
}

impl DisplayCard for CardWithJoker {
    fn to_str(&self) -> String {
        match self {
            CardWithJoker::Card(c) => c.to_str(),
            CardWithJoker::Joker(j) => j.to_str(),
        }
    }

    fn from_str(s: &str) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        if let Ok(r) = Card::from_str(s) {
            return Ok(CardWithJoker::Card(r));
        }

        if let Ok(r) = Joker::from_str(s) {
            return Ok(CardWithJoker::Joker(r));
        }

        Err("Invalid string")
    }

    #[cfg(feature = "pretty")]
    fn to_pretty(&self) -> String {
        match self {
            CardWithJoker::Card(c) => c.to_pretty(),
            CardWithJoker::Joker(j) => j.to_pretty(),
        }
    }

    fn name(&self) -> String {
        match self {
            CardWithJoker::Card(c) => c.name(),
            CardWithJoker::Joker(j) => j.name(),
        }
    }
}

#[derive(Handy, Clone, Deref, DerefMut)]
#[handy_cards(CardWithJoker)]
pub struct Base(Area<CardWithJoker>);

impl Decky<CardWithJoker> for Deck<CardWithJoker> {
    fn new() -> Deck<CardWithJoker> {
        let cards = Card::all_cards()
            .iter()
            .map(|c| CardWithJoker::Card(*c))
            .collect::<Vec<CardWithJoker>>();
        Deck::from_cards(&cards)
    }

    fn push(&mut self, cards: &[CardWithJoker]) {
        self.cards.extend_from_slice(cards);
    }

    fn from_cards(cards: &[CardWithJoker]) -> Deck<CardWithJoker> {
        Deck {
            cards: cards.to_vec(),
            dealt_cards: Vec::with_capacity(cards.len()),
        }
    }

    fn dealt_count(&self) -> usize {
        self.dealt_cards.len()
    }

    fn dealt_cards(&self) -> &[CardWithJoker] {
        self.dealt_cards.as_slice()
    }

    fn deal_one(&mut self) -> Result<CardWithJoker, &'static str> {
        if let Some(card) = self.cards.pop() {
            self.dealt_cards.push(card);
            Ok(card)
        } else {
            Err("No cards left")
        }
    }

    fn reset(&mut self) {
        self.cards.extend(self.dealt_cards.iter().rev());
        self.dealt_cards.clear();
    }
}

trait AddJokers {
    fn add_jokers(self, amount: u8) -> Self;
}

impl AddJokers for Deck<CardWithJoker> {
    fn add_jokers(mut self, amount: u8) -> Self {
        self.cards
            .resize_with(self.cards.len() + amount as usize, || {
                CardWithJoker::Joker(Default::default())
            });
        self
    }
}

fn main() {
    let mut deck = Deck::<CardWithJoker>::new().add_jokers(4);

    let _b = Base::new();

    //println!("{:?}", b);

    // Shuffle the deck
    deck.shuffle();

    // Deal a card
    for _ in 0..10 {
        if let Ok(card) = deck.deal_one() {
            println!("You dealt a {}", card);
        } else {
            panic!("We should have enough cards for this not to happen")
        }
    }

    // Put dealt cards back onto the deck
    deck.reset();
}
