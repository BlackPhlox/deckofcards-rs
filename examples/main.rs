use deckofcards::{Card, Cards, Deck, Decky, DisplayCard, Hand, Handy, SortCards};

fn main() {
    let mut deck = Deck::<Card>::new();

    // Shuffle the deck
    deck.shuffle();

    let mut hand = Hand::new();
    // Deal a card
    for _ in 0..10 {
        if let Ok(card) = deck.deal_one() {
            hand.push_card(card);
            println!("You dealt a {}", card.name());
        } else {
            panic!("We should have enough cards for this not to happen")
        }
    }

    println!("Hand: {}", hand);

    hand.sort_descending_rank_suit();
    println!("Hand Sorted by Desc Rank: {}", hand);

    hand.sort_ascending_rank_suit();
    println!("Hand Sorted by Asc Rank: {}", hand);

    hand.sort_suit_descending_rank();
    println!("Hand Sorted by Suit Desc Rank: {}", hand);

    hand.sort_suit_ascending_rank();
    println!("Hand Sorted by Suit Asc Rank: {}", hand);

    // Put dealt cards back onto the deck
    deck.reset();
}
