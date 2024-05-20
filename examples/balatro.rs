use deckofcards::{card, hand, Area, Card, Cards, Cardy, Color, Deck, Hand, Handy, PlayingCards};
use deref_derive::{Deref, DerefMut};
use handy_derive::Handy;

#[derive(Clone, Debug, PartialEq)]
enum CardType {
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

impl Cardy for CardType {
    fn to_str(&self) -> String {
        match self {
            CardType::Card(_) => todo!(),
            CardType::Joker(j) => j.item.shorthand.to_owned(),
            CardType::Tarot(_) => todo!(),
            CardType::Planet(_) => todo!(),
            CardType::Voucher(_) => todo!(),
        }
    }

    fn from_str(s: &str) -> std::prelude::v1::Result<CardType, &'static str> {
        return Ok(CardType::Joker(Joker {
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
            CardType::Card(_) => todo!(),
            CardType::Joker(j) => j.item.shorthand.to_owned(),
            CardType::Tarot(_) => todo!(),
            CardType::Planet(_) => todo!(),
            CardType::Voucher(_) => todo!(),
        }
    }

    fn name(&self) -> String {
        match self {
            CardType::Card(_) => todo!(),
            CardType::Joker(j) => j.item.name.to_owned(),
            CardType::Tarot(_) => todo!(),
            CardType::Planet(_) => todo!(),
            CardType::Voucher(_) => todo!(),
        }
    }
}

pub struct Base(Area<CardType>);

impl Default for Base {
    fn default() -> Self {
        Self(Area { cards: vec![] })
    }
}

impl Cards<CardType> for Base {
    fn cards(&self) -> &[CardType] {
        self.0.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [CardType] {
        self.0.cards.as_mut_slice()
    }
}

impl Handy<CardType> for Base {
    fn new() -> Self {
        Self::default()
    }

    fn from_hand(hand: &Self) -> Self {
        Self::from_cards(hand.cards())
    }

    fn from_cards(cards: &[CardType]) -> Self {
        Self {
            0: Area {
                cards: Vec::from(cards),
            },
        }
    }

    fn from_strings(card_slice: &[&str]) -> Self {
        let cards = card_slice
            .iter()
            .map(|s| {
                CardType::Joker(Joker {
                    item: Item {
                        name: "".to_string(),
                        shorthand: "".to_string(),
                        base_buy_value: 3,
                        base_sell_value: 2,
                    },
                    jtype: JokerType::GreenJoker(0),
                    edition: Default::default(),
                })
            })
            .collect::<Vec<CardType>>();
        Self { 0: Area { cards } }
    }

    fn push_card(&mut self, card: CardType) {
        self.0.cards.push(card);
    }

    fn push_cards(&mut self, cards: &[CardType]) {
        //self.0.cards.extend(cards);
        todo!()
    }

    fn push_hand(&mut self, other: &Self) {
        //self.0.cards.extend(other.cards());
        todo!()
    }

    fn len(&self) -> usize {
        self.0.cards.len()
    }

    fn clear(&mut self) {
        self.0.cards.clear();
    }

    fn remove(&mut self, index: usize) -> CardType {
        self.0.cards.remove(index)
    }

    fn remove_cards(&mut self, cards: &[CardType]) {
        for c in cards {
            let _ = self.remove_card(c);
        }
    }

    fn remove_all_cards(&mut self, cards: &[CardType]) {
        for c in cards {
            while self.remove_card(c) {}
        }
    }

    fn remove_card(&mut self, card: &CardType) -> bool {
        if let Some(pos) = self.0.cards.iter().position(|c| c == card) {
            let _ = self.0.cards.remove(pos);
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut deck = Deck::new();

    let mut joker_slots = Base::new();
    let mut consumeable_slots = Base::new();
    let mut hand = Base::new();
    let mut playing_hand = Base::new();

    let mut reroll_shop = Base::new();
    let mut pack_shop = Base::new();

    // Put dealt cards back onto the deck
    deck.reset();
}
