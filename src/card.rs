use inquire::{error::InquireResult, Select, Text, DateSelect};

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
        Text::new(&text_block(&self.front)).prompt()?;
        let options = vec![PASS, AGAIN];
        let result = match Select::new(&text_block(&self.back), options).prompt()? {
            PASS => CardResult::Pass,
            AGAIN => CardResult::Again,
            _ => panic!("invalid selection"),
        };
        Ok(result)
    }

    pub fn prompt_edit(&mut self) -> InquireResult<()> {
        loop {
            let options = vec![PREVIEW, EDIT_FRONT, EDIT_BACK, EDIT_DUE, RETURN];

            match Select::new(&self.display(), options).prompt()? {
                PREVIEW => {
                    self.prompt_ask()?;
                }
                EDIT_FRONT => {
                    let front = Text::new(&format!("{} ->", ellipsis(&self.front, TEXT_WIDTH))).prompt()?;
                    self.front = front;
                }
                EDIT_BACK => {
                    let back = Text::new(&format!("{} ->", ellipsis(&self.back, TEXT_WIDTH))).prompt()?;
                    self.back = back;
                }
                EDIT_DUE => {
                    let date = DateSelect::new(&format!("{} ->", self.due)).prompt()?;
                    let time = NaiveTime::from_hms_opt(4, 0, 0).expect("invalid date");
                    self.due = Utc.from_local_datetime(&NaiveDateTime::new(date, time)).single().expect("no single date");
                }
                RETURN => return Ok(()),
                _ => panic!("invalid selection"),
            }
        }
    }
}
