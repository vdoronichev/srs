use inquire::{error::InquireResult, Text};

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

    pub fn prompt_new() -> InquireResult<Self> {
        let name = Text::new("Name:")
            .prompt()?;
        Ok(Self::new(name))
    }

    pub fn display(&self) -> String {
        let cards = match self.cards.len() {
            1 => format!("{} card", 1),
            n => format!("{} cards", n),
        };
        format!("📕 {} - {}", self.name, cards)
    }
}
