use deckofcards::{
    card, hand, Area, Card, Cards, Color, Deck, Decky, DisplayCard, Hand, Handy, SortCards,
};
use deref_derive::{Deref, DerefMut};
use handy_derive::Handy;

#[derive(Clone, Debug, PartialEq)]
pub enum BalatroCard {
    Card(Card),
    Joker(Joker),
    Tarot(Tarot),
    Planet(Planet),
    Voucher(Voucher),
}

#[derive(Clone, Debug, Default, PartialEq)]
enum Edition {
    #[default]
    None,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

#[derive(Clone, Debug, Default)]
enum Seal {
    #[default]
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

#[derive(Clone, Debug, Default)]
enum Enhanced {
    #[default]
    None,
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub shorthand: String,
    pub base_buy_value: u32,
    pub base_sell_value: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum JokerType {
    GreenJoker(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Joker {
    pub item: Item,
    pub jtype: JokerType,
    pub edition: Edition,
}

#[derive(Clone, Debug, PartialEq)]
enum TarotType {
    Fool,
    Magician,
    Death,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tarot {
    pub item: Item,
    pub ttype: TarotType,
}

#[derive(Clone, Debug, PartialEq)]
enum PlanetType {
    Mercury,
    Earth,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Planet {
    pub item: Item,
    pub ptype: PlanetType,
}

#[derive(Clone, Debug, PartialEq)]
enum SpectralType {
    Familiar,
    Grim,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Spectral {
    pub item: Item,
    pub stype: SpectralType,
}

#[derive(Clone, Debug, PartialEq)]
enum VoucherType {
    Overstock,
    Hone,
    Clearance_Sale,
    Reroll_Surplus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Voucher {
    pub item: Item,
    pub vtype: VoucherType,
}

impl DisplayCard for BalatroCard {
    fn to_str(&self) -> String {
        match self {
            BalatroCard::Card(_) => todo!(),
            BalatroCard::Joker(j) => j.item.shorthand.to_owned(),
            BalatroCard::Tarot(_) => todo!(),
            BalatroCard::Planet(_) => todo!(),
            BalatroCard::Voucher(_) => todo!(),
        }
    }

    fn from_str(s: &str) -> std::prelude::v1::Result<BalatroCard, &'static str> {
        return Ok(BalatroCard::Joker(Joker {
            item: Item {
                name: "Green Joker".to_string(),
                shorthand: "GJ".to_string(),
                base_buy_value: 3,
                base_sell_value: 1,
            },
            jtype: JokerType::GreenJoker(0),
            edition: Edition::None,
        }));
    }

    fn to_pretty(&self) -> String {
        match self {
            BalatroCard::Card(_) => todo!(),
            BalatroCard::Joker(j) => j.item.shorthand.to_owned(),
            BalatroCard::Tarot(_) => todo!(),
            BalatroCard::Planet(_) => todo!(),
            BalatroCard::Voucher(_) => todo!(),
        }
    }

    fn name(&self) -> String {
        match self {
            BalatroCard::Card(_) => todo!(),
            BalatroCard::Joker(j) => j.item.name.to_owned(),
            BalatroCard::Tarot(_) => todo!(),
            BalatroCard::Planet(_) => todo!(),
            BalatroCard::Voucher(_) => todo!(),
        }
    }
}

#[derive(Clone, Deref, DerefMut)]
pub struct BlatroArea(Area<BalatroCard>);

impl Default for BlatroArea {
    fn default() -> Self {
        Self(Area { cards: vec![] })
    }
}

impl Cards<BalatroCard> for BlatroArea {
    fn cards(&self) -> &[BalatroCard] {
        self.0.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [BalatroCard] {
        self.0.cards.as_mut_slice()
    }
}

impl Handy<BalatroCard> for BlatroArea {
    fn new() -> Self {
        Self(Area { cards: vec![] })
    }

    fn from_hand(hand: &Self) -> Self {
        Self::from_cards(hand.cards())
    }

    fn from_cards(cards: &[BalatroCard]) -> Self {
        Self {
            0: Area {
                cards: Vec::from(cards),
            },
        }
    }

    fn from_strings(card_slice: &[&str]) -> Self {
        todo!()
    }

    fn push_card(&mut self, card: BalatroCard) {
        self.0.cards.push(card);
    }

    fn push_cards(&mut self, cards: &[BalatroCard]) {
        self.0.cards.extend_from_slice(cards);
    }

    fn push_hand(&mut self, other: &Self) {
        self.0.cards.extend_from_slice(other.cards());
    }

    fn len(&self) -> usize {
        self.0.cards.len()
    }

    fn clear(&mut self) {
        self.0.cards.clear();
    }

    fn remove(&mut self, index: usize) -> BalatroCard {
        self.0.cards.remove(index)
    }

    fn remove_cards(&mut self, cards: &[BalatroCard]) {
        for c in cards {
            let _ = self.remove_card(c);
        }
    }

    fn remove_all_cards(&mut self, cards: &[BalatroCard]) {
        for c in cards {
            while self.remove_card(c) {}
        }
    }

    fn remove_card(&mut self, card: &BalatroCard) -> bool {
        if let Some(pos) = self.0.cards.iter().position(|c| c == card) {
            let _ = self.0.cards.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Decky<BalatroCard> for Deck<BalatroCard> {
    fn new() -> Deck<BalatroCard> {
        let cards = Card::all_cards()
            .iter()
            .map(|c| BalatroCard::Card(*c))
            .collect::<Vec<BalatroCard>>();
        Deck::from_cards(&cards)
    }

    fn push(&mut self, cards: &[BalatroCard]) {
        self.cards.extend_from_slice(cards);
    }

    fn from_cards(cards: &[BalatroCard]) -> Deck<BalatroCard> {
        Deck {
            cards: cards.to_vec(),
            dealt_cards: Vec::with_capacity(cards.len()),
        }
    }

    fn dealt_count(&self) -> usize {
        self.dealt_cards.len()
    }

    fn dealt_cards(&self) -> &[BalatroCard] {
        self.dealt_cards.as_slice()
    }

    fn deal_one(&mut self) -> Result<BalatroCard, &'static str> {
        if let Some(card) = self.cards.pop() {
            self.dealt_cards.push(card.clone());
            Ok(card)
        } else {
            Err("No cards left")
        }
    }

    fn reset(&mut self) {
        let cards = self
            .dealt_cards
            .iter()
            .rev()
            .map(|dc| dc.clone())
            .collect::<Vec<BalatroCard>>();
        self.cards.extend(cards);
        self.dealt_cards.clear();
    }
}

fn main() {
    let mut card_deck = Deck::<BalatroCard>::new();

    let mut joker_deck = Deck::<BalatroCard>::empty();
    let mut voucher_deck = Deck::<BalatroCard>::empty();

    let mut joker_slots = BlatroArea::new();
    let mut consumeable_slots = BlatroArea::new();
    let mut hand = BlatroArea::new();
    let mut playing_hand = BlatroArea::new();

    let mut reroll_shop = BlatroArea::new();
    let mut pack_shop = BlatroArea::new();

    // Put dealt cards back onto the deck
    card_deck.reset();
}
