use inquire::Text;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Card {
    pub front: String,
    pub back: String,
    pub due: NaiveDate,
}

impl Card {
    pub fn new(front: impl Into<String>, back: impl Into<String>) -> Self {
        Self {
            front: front.into(),
            back: back.into(),
            due: Utc::now().date_naive(),
        }
    }

    pub fn prompt_new() -> InquireResult<Self> {
        let front = Text::new(ENTER_CARD_FRONT)
            .with_help_message(HELP_CARD_FRONT)
            .prompt()?;
        let back = Text::new(ENTER_CARD_BACK)
            .with_help_message(HELP_CARD_BACK)
            .prompt()?;
        Ok(Self::new(front, back))
    }

    pub fn display_name(&self) -> String {
        format!(
            "🃏 {} / {}",
            ellipsis(&self.front, TEXT_WIDTH),
            ellipsis(&self.back, TEXT_WIDTH)
        )
    }
}
