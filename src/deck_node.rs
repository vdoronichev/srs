use std::collections::HashMap;

use inquire::{error::InquireResult, Select, Text, MultiSelect};

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

    pub fn prompt_new() -> InquireResult<Self> {
        let name = Text::new("Name:")
            .prompt()?;
        Ok(Self::new(name))
    }

    pub fn display(&self) -> String {
        let entries = match self.entries.len() {
            1 => format!("{} entry", 1),
            n => format!("{} entries", n),
        };
        format!("📁 {} - {}", self.name, entries)
    }

    fn entries_options(&self) -> (Vec<String>, HashMap<String, usize>) {
        let mut options: Vec<String> = self.entries
            .iter()
            .map(|entry| entry.display())
            .collect();
        let option_to_index: HashMap<String, usize> = options
            .iter()
            .enumerate()
            .map(|(index, entry)| (format!("{}. {}", index + 1, entry), index))
            .collect();
        options.insert(0, RETURN.into());
        (options, option_to_index)
    }

    pub fn prompt_edit(&mut self) -> InquireResult<()> {
        loop {
            let options = if self.entries.is_empty() {
                vec![ADD_DECK, ADD_SET, RENAME_SET, RETURN]
            } else {
                vec![ADD_DECK, ADD_SET, ENTRIES, REMOVE_ENTRIES, RENAME_SET, RETURN]
            };

            match Select::new(&self.display(), options).prompt()? {
                ADD_DECK => {
                    let deck = Deck::prompt_new()?;
                    self.entries.push(DeckNode::Leaf(deck));
                }
                ADD_SET => {
                    let set = DeckSet::prompt_new()?;
                    self.entries.push(DeckNode::Inner(set));
                }
                ENTRIES => {
                    loop {
                        let (options, option_to_index) = self.entries_options();
                        let opt_return = RETURN.to_owned();
                        match Select::new(&self.display(), options).prompt()? {
                            opt_return => break,
                            option => {
                                let index = option_to_index[&option];
                            }
                        }
                    }
                }
                REMOVE_ENTRIES => {
                    let (options, option_to_index) = self.entries_options();
                }
                RENAME_SET => {
                    let name = Text::new(&format!("{} ->", self.name)).prompt()?;
                    self.name = name
                }
                RETURN => return Ok(()),
                _ => panic!("invalid selection"),
            }
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

    pub fn display(&self) -> String {
        match self {
            Self::Inner(set) => set.display(),
            Self::Leaf(deck) => deck.display(),
        }
    }
}
