use std::collections::{HashMap, VecDeque};

use inquire::{error::InquireResult, Text};

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
        format!("📁 {} ({})", self.name, self.entries.len())
    }
}

#[derive(Debug, Clone)]
pub enum DeckNode {
    Inner { set: DeckSet, expanded: bool },
    Leaf(Deck),
}

impl DeckNode {
    pub fn of_set(set: DeckSet) -> Self {
        Self::Inner {
            set,
            expanded: true,
        }
    }

    pub fn of_deck(deck: Deck) -> Self {
        Self::Leaf(deck)
    }

    pub fn at<'a>(&'a self, path: DeckPath) -> Option<&'a DeckNode> {
        let mut path = path.clone();
        let Some(first) = path.0.pop_front() else { return Some(self); };
        match self {
            Self::Inner { set, .. } => set.entries.get(first).and_then(|child| child.at(path)),
            Self::Leaf(_) => None,
        }
    }

    pub fn at_mut<'a>(&'a mut self, path: DeckPath) -> Option<&'a mut DeckNode> {
        let mut path = path.clone();
        let Some(first) = path.0.pop_front() else { return Some(self); };
        match self {
            Self::Inner { set, .. } => set
                .entries
                .get_mut(first)
                .and_then(|child| child.at_mut(path)),
            Self::Leaf(_) => None,
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Inner { set, .. } => set.display(),
            Self::Leaf(deck) => deck.display(),
        }
    }

    pub fn prompt_options(&self) -> (Vec<String>, HashMap<String, (DeckPath, usize)>) {
        let mut options: Vec<(String, DeckPath)> = Vec::new();

        fn build(node: &DeckNode, options: &mut Vec<(String, DeckPath)>, path: VecDeque<usize>) {
            let indent = "  ".repeat(path.len());
            let text = format!("{}{}", indent, node.display());
            options.push((text, DeckPath(path.clone())));

            let DeckNode::Inner { set, expanded: true } = node else { return; };
            for (i, child) in set.entries.iter().enumerate() {
                let mut path = path.clone();
                path.push_back(i);
                build(child, options, path);
            }
        }

        build(self, &mut options, VecDeque::new());

        let options: Vec<(String, DeckPath)> = options
            .into_iter()
            .enumerate()
            .map(|(i, (key, v))| (format!("{}. {}", i, key), v))
            .collect();
        (
            options.iter().map(|(key, _)| key.to_owned()).collect(),
            options
                .iter()
                .enumerate()
                .map(|(i, (key, path))| (key.clone(), (path.clone(), i)))
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct DeckPath(pub VecDeque<usize>);
