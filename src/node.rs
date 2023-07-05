use std::{fmt, collections::VecDeque};

use inquire::{Text, Select, MultiSelect};

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

    pub fn prompt_options(&self) -> Vec<NodePromptOption> {
        fn build(
            this: &DeckNode,
            options: &mut Vec<NodePromptOption>,
            path: Vec<usize>,
        ) {
            options.push(NodePromptOption {
                action: match this {
                    DeckNode::Set { .. } => NodeAction::ToggleExpanded,
                    DeckNode::Deck { .. } => NodeAction::Edit,
                },
                name: this.name(),
                path: DeckPath::new(path.clone()),
            });

            match this {
                DeckNode::Set { entries, expanded: true, .. } => {
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

    pub fn prompt_edit(&mut self, action: NodeAction) -> InquireResult<()> {
        loop {
            let title = &self.name();
            match self {
                Self::Set { name, entries, expanded, .. } => {
                    match action {
                        NodeAction::ToggleExpanded => {
                            *expanded = !*expanded;
                        }
                        NodeAction::Edit => {
                            enum Option {
                                AddDeck,
                                AddSet,
                                Rename,
                                Remove,
                                Back,
                            }
    
                            impl fmt::Display for Option {
                                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                                    write!(f, "{}", match self {
                                        Self::AddDeck => ADD_DECK,
                                        Self::AddSet => ADD_SET,
                                        Self::Rename => RENAME,
                                        Self::Remove => REMOVE,
                                        Self::Back => BACK,
                                    })
                                }
                            }
    
                            match Select::new(
                                title,
                                if entries.is_empty() {
                                    vec![
                                        Option::AddDeck,
                                        Option::AddSet,
                                        Option::Rename,
                                        Option::Back,
                                    ]
                                } else {
                                    vec![
                                        Option::AddDeck,
                                        Option::AddSet,
                                        Option::Rename,
                                        Option::Remove,
                                        Option::Back,
                                    ]
                                },
                            ).prompt()? {
                                Option::AddDeck => {
                                    entries.push(Self::prompt_deck()?);
                                }
                                Option::AddSet => {
                                    entries.push(Self::prompt_set()?);
                                }
                                Option::Rename => {
                                    *name = Text::new(&format!("{} ->", name)).prompt()?;
                                }
                                Option::Remove => {
                                    struct Entry {
                                        index: usize,
                                        name: String,
                                    }

                                    let options: Vec<Entry> = entries
                                        .iter()
                                        .enumerate()
                                        .map(|(i, x)| Entry {
                                            index: i,
                                            name: x.name(),
                                        })
                                        .collect();

                                    impl fmt::Display for Entry {
                                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                                            write!(f, "{}", self.name)
                                        }
                                    }

                                    let x = MultiSelect::new(
                                        "Remove",
                                        options,
                                    ).prompt()?;
                                }
                                Option::Back => return Ok(()),
                            }
                        }
                    }
                }
                Self::Deck { cards, .. } => {
                    enum Option {
                        AddCard,
                        Rename,
                        Remove,
                        Back,
                    }

                    Select::new(&self.name(), vec!["abc"]).prompt()?;
                    return Ok(());
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

impl std::fmt::Display for NodePromptOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = "  ".repeat(self.path.0.len());
        write!(f, "{}{}", indent, self.name)
    }
}
