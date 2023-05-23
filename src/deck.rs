use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cards: Vec::new(),
        }
    }
}
