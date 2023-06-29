use std::collections::{HashMap, VecDeque};

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
        match self {
            Self::Set { entries, .. } => {
                
            }
            Self::Deck { cards, .. } => {
                vec![]
            }
        }

        let options: Vec<(String, DeckPath)> = Vec::new();

        fn build(
            node: &DeckNode,
            options: &mut Vec<(String, DeckPath)>,
        ) {

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
pub struct DeckPromptOption {
    pub name: String,
    pub path: DeckPath,
}

impl std::fmt::Display for DeckPromptOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "  ".repeat(self.path.0.len()), self.name)
    }
}
