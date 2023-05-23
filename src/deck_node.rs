use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeckSet {
    pub entries: Vec<DeckNode>,
}

#[derive(Debug, Clone)]
pub enum DeckNode {
    Inner(DeckSet),
    Leaf(Deck),
}
