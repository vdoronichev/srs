use std::collections::HashMap;

use inquire::{error::InquireResult, Text, Select, MultiSelect};

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
        let name = Text::new("Name:").prompt()?;
        Ok(Self::new(name))
    }

    pub fn display(&self) -> String {
        let cards = match self.cards.len() {
            1 => format!("{} card", 1),
            n => format!("{} cards", n),
        };
        format!("📕 {} - {}", self.name, cards)
    }

    fn cards_options(&self) -> (Vec<String>, HashMap<String, usize>) {
        let options: Vec<String> = self.cards
            .iter()
            .enumerate()
            .map(|(index, entry)| format!("{}. {}", index + 1, entry.display()))
            .collect();
        let option_to_index: HashMap<String, usize> = options
            .iter()
            .enumerate()
            .map(|(index, entry)| (entry.clone(), index))
            .collect();
        (options, option_to_index)
    }

    pub fn prompt_edit(&mut self) -> InquireResult<()> {
        loop {
            let options = if self.cards.is_empty() {
                vec![ADD_CARD, RENAME_DECK, RETURN]
            } else {
                vec![ADD_CARD, LIST_CARDS, REMOVE_CARDS, RENAME_DECK, RETURN]
            };

            match Select::new(&self.display(), options).prompt()? {
                ADD_CARD => {
                    let card = Card::prompt_new()?;
                    self.cards.push(card);
                }
                LIST_CARDS => loop {
                    let (mut options, option_to_index) = self.cards_options();
                    options.insert(0, RETURN.into());
                    let option = Select::new(&self.display(), options.clone()).prompt()?;
                    let Some(index) = option_to_index.get(&option) else {
                        break;
                    };
                    let option = &mut self.cards[*index];
                    option.prompt_edit()?
                }
                REMOVE_CARDS => {
                    let (options, option_to_index) = self.cards_options();
                    let options = MultiSelect::new("Remove cards:", options.clone()).prompt()?;
                    let mut indices_to_remove: Vec<_> = options
                        .into_iter()
                        .map(|select| option_to_index[&select])
                        .collect();
                    sort(&mut indices_to_remove);

                    if indices_to_remove.is_empty() || prompt_confirm()? {
                        // indices must be reversed so that we don't remove invalid indices
                        // since all indices get shifted after we remove one
                        for index in indices_to_remove.into_iter().rev() {
                            self.cards.remove(index);
                        }
                    }
                }
                RENAME_DECK => {
                    let name = Text::new(&format!("{} ->", self.name)).prompt()?;
                    self.name = name;
                }
                RETURN => return Ok(()),
                _ => panic!("invalid selection"),
            }
        }
    }
}
