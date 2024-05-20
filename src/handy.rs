pub trait Handy<C /*: Cards<C> = Self*/> {
    fn new() -> Self;
    fn from_hand(hand: &Self) -> Self;
    fn from_cards(cards: &[C]) -> Self;
    fn from_strings(card_slice: &[&str]) -> Self;
    fn push_card(&mut self, card: C);
    fn push_cards(&mut self, cards: &[C]);
    fn push_hand(&mut self, other: &Self);
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn remove(&mut self, index: usize) -> C;
    fn remove_cards(&mut self, cards: &[C]);
    fn remove_all_cards(&mut self, cards: &[C]);
    fn remove_card(&mut self, card: &C) -> bool;
}
