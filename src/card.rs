use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Card {
    pub front: String,
    pub back: String,
    pub due: DateTime<Utc>,
}

impl Card {
    pub fn new(front: impl Into<String>, back: impl Into<String>) -> Self {
        Self {
            front: front.into(),
            back: back.into(),
            due: Utc::now(),
        }
    }
}
