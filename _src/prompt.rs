use inquire::{error::InquireResult, Select};

pub const NO: &str = "No";
pub const YES: &str = "Yes";

pub const MAIN_MENU: &str = "Main Menu";
pub const RETURN: &str = "Return";
pub const EXIT: &str = "Exit";
pub const STATS: &str = "Stats";

pub const ADD_DECK: &str = "Add Deck";
pub const ADD_SET: &str = "Add Set";
pub const REMOVE_ENTRIES: &str = "Remove Entries";
pub const RENAME_SET: &str = "Rename Set";
pub const ENTRIES: &str = "Entries";

pub const ADD_CARD: &str = "Add Card";
pub const LIST_CARDS: &str = "List Cards";
pub const REMOVE_CARDS: &str = "Remove Cards";
pub const RENAME_DECK: &str = "Rename Deck";
pub const CARDS: &str = "Cards";

pub const PREVIEW: &str = "Preview";
pub const EDIT_FRONT: &str = "Edit Front";
pub const EDIT_BACK: &str = "Edit Back";
pub const EDIT_DUE: &str = "Edit Due Date";
pub const CARD_FRONT_HELP: &str = "the text that is used as the prompt";
pub const CARD_BACK_HELP: &str = "the answer to that prompt";
pub const PASS: &str = "Pass";
pub const AGAIN: &str = "Again";

pub fn prompt_confirm() -> InquireResult<bool> {
    return Ok(
        match Select::new("Are you sure?", vec![NO, YES]).prompt()? {
            NO => false,
            YES => true,
            _ => panic!("invalid selection"),
        },
    );
}
