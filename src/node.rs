use std::{collections::VecDeque, fmt};

use inquire::{MultiSelect, Select, Text};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum DeckNode {
    Set {
        name: String,
        entries: Vec<DeckNode>,
        expanded: bool,
    },
    Deck {
        name: String,
        cards: Vec<Card>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum NodeAction {
    Edit,
    ToggleExpanded,
}

impl DeckNode {
    pub fn set(name: impl Into<String>, entries: impl Into<Vec<Self>>) -> Self {
        Self::Set {
            name: name.into(),
            entries: entries.into(),
            expanded: true,
        }
    }

    pub fn deck(name: impl Into<String>, cards: impl Into<Vec<Card>>) -> Self {
        Self::Deck {
            name: name.into(),
            cards: cards.into(),
        }
    }

    pub fn prompt_set() -> InquireResult<Self> {
        let name = Text::new(ENTER_SET_NAME).prompt()?;
        Ok(Self::set(name, []))
    }

    pub fn prompt_deck() -> InquireResult<Self> {
        let name = Text::new(ENTER_DECK_NAME).prompt()?;
        Ok(Self::deck(name, []))
    }

    pub fn at(&self, path: DeckPath) -> Option<&Self> {
        let mut path = path.clone();
        let Some(next_key) = path.0.pop_front() else {
            return Some(self);
        };

        match self {
            Self::Set { entries, .. } => entries.get(next_key).and_then(|next| next.at(path)),
            Self::Deck { .. } => None,
        }
    }

    pub fn at_mut(&mut self, path: DeckPath) -> Option<&mut Self> {
        let mut path = path.clone();
        let Some(next_key) = path.0.pop_front() else {
            return Some(self);
        };

        match self {
            Self::Set { entries, .. } => {
                entries.get_mut(next_key).and_then(|next| next.at_mut(path))
            }
            Self::Deck { .. } => None,
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            Self::Set {
                name,
                entries,
                expanded,
            } => {
                let icon = if *expanded { "📂" } else { "📁" };
                format!("{} {} ({})", icon, name, entries.len())
            }
            Self::Deck { name, cards } => {
                format!("📕 {} ({})", name, cards.len())
            }
        }
    }

    pub fn prompt_options(&self) -> Vec<NodePromptOption> {
        fn build(this: &DeckNode, options: &mut Vec<NodePromptOption>, path: Vec<usize>) {
            options.push(NodePromptOption {
                action: match this {
                    DeckNode::Set { .. } => NodeAction::ToggleExpanded,
                    DeckNode::Deck { .. } => NodeAction::Edit,
                },
                name: this.display_name(),
                path: DeckPath::new(path.clone()),
            });

            match this {
                DeckNode::Set {
                    entries,
                    expanded: true,
                    ..
                } => {
                    for (i, child) in entries.iter().enumerate() {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        build(child, options, new_path);
                    }
                    options.push(NodePromptOption {
                        action: NodeAction::Edit,
                        name: "  ⚙️".to_owned(),
                        path: DeckPath::new(path.clone()),
                    });
                }
                _ => {}
            }
        }

        let mut options: Vec<NodePromptOption> = Vec::new();
        build(self, &mut options, Vec::new());
        options
    }

    pub fn prompt_select(&mut self, action: NodeAction) -> InquireResult<()> {
        match action {
            NodeAction::ToggleExpanded => {
                let Self::Set { expanded, .. } = self else {
                    panic!("ToggleExpanded is only applicable to Set's");
                };
                *expanded = !*expanded;
                Ok(())
            }
            NodeAction::Edit => self.prompt_edit(),
        }
    }

    pub fn prompt_edit(&mut self) -> InquireResult<()> {
        struct RemoveEntry {
            index: usize,
            name: String,
        }

        impl fmt::Display for RemoveEntry {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.name)
            }
        }

        fn sort_removal(vec: &mut Vec<RemoveEntry>) {
            sort(vec, |a, b| a.index.cmp(&b.index));
        }

        loop {
            let display_name = &self.display_name();
            match self {
                Self::Set { name, entries, .. } => {
                    enum Selection {
                        AddDeck,
                        AddSet,
                        Rename,
                        RemoveEntries,
                        Back,
                    }

                    impl fmt::Display for Selection {
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                            write!(
                                f,
                                "{}",
                                match self {
                                    Self::AddDeck => ADD_DECK,
                                    Self::AddSet => ADD_SET,
                                    Self::Rename => RENAME,
                                    Self::RemoveEntries => REMOVE_ENTRIES,
                                    Self::Back => BACK,
                                },
                            )
                        }
                    }

                    match Select::new(
                        display_name,
                        if entries.is_empty() {
                            vec![
                                Selection::AddDeck,
                                Selection::AddSet,
                                Selection::Rename,
                                Selection::Back,
                            ]
                        } else {
                            vec![
                                Selection::AddDeck,
                                Selection::AddSet,
                                Selection::Rename,
                                Selection::RemoveEntries,
                                Selection::Back,
                            ]
                        },
                    )
                    .prompt()?
                    {
                        Selection::AddDeck => {
                            entries.push(Self::prompt_deck()?);
                        }
                        Selection::AddSet => {
                            entries.push(Self::prompt_set()?);
                        }
                        Selection::Rename => {
                            *name = prompt_rename(&display_name)?;
                        }
                        Selection::RemoveEntries => {
                            let options: Vec<RemoveEntry> = entries
                                .iter()
                                .enumerate()
                                .map(|(i, x)| RemoveEntry {
                                    index: i,
                                    name: x.display_name(),
                                })
                                .collect();

                            let mut to_remove = MultiSelect::new(ENTER_REMOVE, options).prompt()?;
                            if to_remove.is_empty() {
                                continue;
                            }
                            if !prompt_confirm()? {
                                continue;
                            }

                            sort_removal(&mut to_remove);
                            for entry in to_remove.into_iter().rev() {
                                entries.remove(entry.index);
                            }
                        }
                        Selection::Back => return Ok(()),
                    }
                }
                Self::Deck { name, cards, .. } => {
                    enum Selection {
                        AddCard,
                        Rename,
                        RemoveCards,
                        Back,
                    }

                    impl fmt::Display for Selection {
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                            write!(
                                f,
                                "{}",
                                match self {
                                    Self::AddCard => ADD_CARD,
                                    Self::Rename => RENAME,
                                    Self::RemoveCards => REMOVE_CARDS,
                                    Self::Back => BACK,
                                },
                            )
                        }
                    }

                    match Select::new(
                        display_name,
                        if cards.is_empty() {
                            vec![Selection::AddCard, Selection::Rename, Selection::Back]
                        } else {
                            vec![
                                Selection::AddCard,
                                Selection::Rename,
                                Selection::RemoveCards,
                                Selection::Back,
                            ]
                        },
                    )
                    .prompt()?
                    {
                        Selection::AddCard => {
                            cards.push(Card::prompt_new()?);
                        }
                        Selection::Rename => {
                            *name = prompt_rename(display_name)?;
                        }
                        Selection::RemoveCards => {
                            let options: Vec<RemoveEntry> = cards
                                .iter()
                                .enumerate()
                                .map(|(i, x)| RemoveEntry {
                                    index: i,
                                    name: x.display_name(),
                                })
                                .collect();

                            let mut to_remove = MultiSelect::new(ENTER_REMOVE, options).prompt()?;
                            if to_remove.is_empty() {
                                continue;
                            }
                            if !prompt_confirm()? {
                                continue;
                            }

                            sort_removal(&mut to_remove);
                            for entry in to_remove.into_iter().rev() {
                                cards.remove(entry.index);
                            }
                        }
                        Selection::Back => return Ok(()),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeckPath(pub VecDeque<usize>);

impl DeckPath {
    pub fn new(path: impl Into<VecDeque<usize>>) -> Self {
        Self(path.into())
    }
}

#[derive(Clone, Debug)]
pub struct NodePromptOption {
    pub action: NodeAction,
    pub name: String,
    pub path: DeckPath,
}

impl fmt::Display for NodePromptOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "  ".repeat(self.path.0.len());
        write!(f, "{}{}", indent, self.name)
    }
}

fn prompt_rename(display_name: &str) -> InquireResult<String> {
    Text::new(&format!("{} ->", display_name)).prompt()
}
