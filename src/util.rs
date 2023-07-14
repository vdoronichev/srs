use std::{fmt, cmp::Ordering};

use inquire::{error::InquireResult, Select};
use unicode_truncate::UnicodeTruncateStr;

pub const TEXT_WIDTH: usize = 30;

pub fn prompt_confirm() -> InquireResult<bool> {
    enum Selection {
        No,
        Yes,
    }

    impl fmt::Display for Selection {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::No => NO,
                    Self::Yes => YES,
                }
            )
        }
    }

    let opt = Select::new(ASK_CONFIRM, vec![Selection::No, Selection::Yes]).prompt()?;
    Ok(match opt {
        Selection::No => false,
        Selection::Yes => true,
    })
}

pub fn ellipsis(str: &str, width: usize) -> String {
    let truncated = str.unicode_truncate(width).0;
    if truncated.len() < str.len() {
        format!("{}...", truncated)
    } else {
        str.to_owned()
    }
}

pub fn sort<T, F>(vec: &mut Vec<T>, compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    vec.sort_by(compare);
}

pub const MAIN_MENU: &str = "Main Menu";
pub const STATS: &str = "Stats";
pub const QUIT: &str = "Quit";

pub const RENAME: &str = "Rename";
pub const BACK: &str = "Back";

pub const ADD_DECK: &str = "Add Deck";
pub const ADD_SET: &str = "Add Set";
pub const REMOVE_ENTRIES: &str = "Remove Entries";

pub const ADD_CARD: &str = "Add Card";
pub const REMOVE_CARDS: &str = "Remove Cards";

pub const ENTER_REMOVE: &str = "Remove:";

pub const ENTER_DECK_NAME: &str = "Deck name:";
pub const ENTER_SET_NAME: &str = "Set name:";

pub const ENTER_CARD_FRONT: &str = "Front:";
pub const HELP_CARD_FRONT: &str = "the text initially revealed to you";

pub const ENTER_CARD_BACK: &str = "Back:";
pub const HELP_CARD_BACK: &str = "the text you are shown afterwards";

pub const ASK_CONFIRM: &str = "Are you sure?";
pub const NO: &str = "No";
pub const YES: &str = "Yes";
