use srs::prelude::*;

fn main() {
    let mut deck_set = DeckSet::new("Root Deck Set");

    let mut deck = Deck::new("Computer Science");
    deck.cards.push(Card::new("What is RAM?", "Random access memory"));
    
    deck_set.entries.push(DeckNode::Leaf(deck));

    println!("{:?}", deck_set);
}
