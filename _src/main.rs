use inquire::{error::InquireResult, Select};
use srs::prelude::*;

pub fn prompt_menu(root: &mut DeckNode) -> InquireResult<()> {
    let mut last_cursor = 0;
    loop {
        let (mut options, option_map) = root.prompt_options();

        options.push(STATS.to_owned());
        options.push(EXIT.to_owned());

        match Select::new(MAIN_MENU.into(), options)
            // guaranteed to be safe as long as collapsing/expanding sets
            // can only modify entries under the current one
            .with_starting_cursor(last_cursor)
            .prompt()?
            .as_str()
        {
            EXIT => return Ok(()),
            STATS => return Ok(()),
            key => {
                let (path, i) = option_map[&key.to_owned()].clone();
                last_cursor = i;
                let target = root.at_mut(path).expect("invalid node selected");
                match target {
                    DeckNode::Inner { expanded, .. } => {
                        *expanded = !*expanded;
                    }
                    DeckNode::Leaf(deck) => {
                        
                    }
                }
            }
        }
    }
}

fn main() {
    let mut root = DeckSet::new("Decks");

    let mut french = DeckSet::new("French");
    french.entries.push(DeckNode::of_deck(Deck::new("Vocab")));
    french.entries.push(DeckNode::of_deck(Deck::new("Grammar")));
    root.entries.push(DeckNode::of_set(french));

    root.entries.push(DeckNode::of_deck(Deck::new("Comp Sci")));
    let mut root = DeckNode::of_set(root);

    let _ = prompt_menu(&mut root);
}
