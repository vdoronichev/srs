use std::collections::HashMap;

use inquire::{error::InquireResult, MultiSelect, Select, Text};

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
        let name = Text::new("Name:").prompt()?;
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
        let options: Vec<String> = self
            .entries
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
            let options = if self.entries.is_empty() {
                vec![ADD_DECK, ADD_SET, RENAME_SET, RETURN]
            } else {
                vec![
                    ADD_DECK,
                    ADD_SET,
                    ENTRIES,
                    REMOVE_ENTRIES,
                    RENAME_SET,
                    RETURN,
                ]
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
                ENTRIES => loop {
                    let (mut options, option_to_index) = self.entries_options();
                    options.insert(0, RETURN.into());
                    let option = Select::new(&self.display(), options.clone()).prompt()?;
                    let Some(index) = option_to_index.get(&option) else {
                        break;
                    };
                    let option = &mut self.entries[*index];
                    option.prompt_edit()?
                },
                REMOVE_ENTRIES => {
                    let (options, option_to_index) = self.entries_options();
                    let options = MultiSelect::new("Remove entries:", options.clone()).prompt()?;
                    let mut indices_to_remove: Vec<_> = options
                        .into_iter()
                        .map(|select| option_to_index[&select])
                        .collect();
                    sort(&mut indices_to_remove);

                    if indices_to_remove.is_empty() || prompt_confirm()? {
                        // indices must be reversed so that we don't remove invalid indices
                        // since all indices get shifted after we remove one
                        for index in indices_to_remove.into_iter().rev() {
                            self.entries.remove(index);
                        }
                    }
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

    pub fn prompt_edit(&mut self) -> InquireResult<()> {
        match self {
            Self::Inner(set) => set.prompt_edit(),
            Self::Leaf(deck) => deck.prompt_edit(),
        }
    }
}
