use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeckSet {
    pub name: String,
    pub entries: Vec<DeckNode>,
}

impl DeckSet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeckNode {
    Inner(DeckSet),
    Leaf(Deck),
}

impl DeckNode {
    pub fn new_set(name: impl Into<String>) -> Self {
        Self::Inner(DeckSet::new(name))
    }

    pub fn new_deck(name: impl Into<String>) -> Self {
        Self::Leaf(Deck::new(name))
    }
}
