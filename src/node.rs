use std::collections::{HashMap, VecDeque};

use inquire::Text;

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

impl DeckNode {
    pub fn set(name: impl Into<String>, entries: impl Into<Vec<DeckNode>>) -> Self {
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

    pub fn prompt_set() -> InquireResult<DeckNode> {
        let name = Text::new("Enter set name:").prompt()?;
        Ok(Self::set(name, []))
    }

    pub fn prompt_deck() -> InquireResult<DeckNode> {
        let name = Text::new("Enter deck name:").prompt()?;
        Ok(Self::deck(name, []))
    }

    pub fn at(&self, path: DeckPath) -> Option<&DeckNode> {
        let mut path = path.clone();
        let Some(next_key) = path.0.pop_front() else {
            return Some(self);
        };

        match self {
            Self::Set { entries, .. } => {
                entries.get(next_key).and_then(|next| next.at(path))
            }
            Self::Deck { .. } => None,
        }
    }

    pub fn at_mut(&mut self, path: DeckPath) -> Option<&mut DeckNode> {
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

    pub fn name(&self) -> String {
        match self {
            Self::Set { name, entries, expanded } => {
                let icon = if *expanded { "📂" } else { "📁" };
                format!("{} {} ({})", icon, name, entries.len())
            },
            Self::Deck { name, cards } => format!("📕 {} ({})", name, cards.len())
        }
    }

    pub fn prompt_options(&self) -> Vec<DeckPromptOption> {
        fn build(
            this: &DeckNode,
            options: &mut Vec<DeckPromptOption>,
            path: Vec<usize>,
        ) {
            options.push(DeckPromptOption {
                action: DeckPromptAction::Default {
                    name: this.name(),
                },
                path: DeckPath::new(path.clone()),
            });

            match this {
                DeckNode::Set { entries, expanded: true, .. } => {
                    for (i, child) in entries.iter().enumerate() {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        build(child, options, new_path);
                    }
                    options.push(DeckPromptOption {
                        action: DeckPromptAction::AddDeck,
                        path: DeckPath::new(path.clone()),
                    });
                    options.push(DeckPromptOption {
                        action: DeckPromptAction::AddSet,
                        path: DeckPath::new(path.clone()),
                    });
                }
                _ => {}
            }
        }

        let mut options: Vec<DeckPromptOption> = Vec::new();
        build(self, &mut options, Vec::new());
        options
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
pub struct DeckPromptOption {
    pub action: DeckPromptAction,
    pub path: DeckPath,
}

#[derive(Clone, Debug)]
pub enum DeckPromptAction {
    Default {
        name: String,
    },
    AddDeck,
    AddSet,
}

impl std::fmt::Display for DeckPromptOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = "  ".repeat(self.path.0.len());
        match &self.action {
            DeckPromptAction::Default { name } => write!(f, "{}{}", indent, name),
            DeckPromptAction::AddDeck => write!(f, "{}  ➕ Add Deck", indent),
            DeckPromptAction::AddSet => write!(f, "{}  ➕ Add Set", indent),
        }
    }
}
