use inquire::{error::InquireResult, Text, Select};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Card {
    pub front: String,
    pub back: String,
    pub due: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum CardResult {
    Pass,
    Again,
}

impl Card {
    pub fn new(front: impl Into<String>, back: impl Into<String>) -> Self {
        Self {
            front: front.into(),
            back: back.into(),
            due: Utc::now(),
        }
    }

    pub fn prompt_new() -> InquireResult<Self> {
        let front = Text::new("Front:")
            .with_help_message(CARD_FRONT_HELP)
            .prompt()?;
        let back = Text::new("Back:")
            .with_help_message(CARD_BACK_HELP)
            .prompt()?;
        Ok(Self::new(front, back))
    }

    pub fn display(&self) -> String {
        format!("🃏 {} / {}", self.front, self.back)
    }

    pub fn prompt_ask(&self) -> InquireResult<CardResult> {
        Text::new(&text_block(self.front)).prompt()?;
        let options = vec![PASS, AGAIN];
        let result = match Select::new(&text_block(self.back), options).prompt()? {
            PASS => CardResult::Pass,
            AGAIN => CardResult::Again,
        };
        Ok(result)
    }

    pub fn prompt_edit(&mut self) -> InquireResult<()> {
        loop {
            let options = vec![
                PREVIEW,
                EDIT_FRONT,
                EDIT_BACK,
                EDIT_DUE,
            ];

            match Select::new(&self.display(), options).prompt()? {
                PREVIEW => {
                    self.prompt_ask();
                }
                _ => panic!("invalid selection")
            }
        }
    }
}
