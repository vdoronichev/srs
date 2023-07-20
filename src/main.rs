use inquire::Select;
use srs::prelude::*;
use std::fmt;

/// The main entry point of the program, run when the user opens the app.
fn main() {
    // Set up the root DeckNode, holding the hierarchy of user-created decks
    let mut root = DeckNode::set(
        "All Decks",
        [
            DeckNode::set(
                "French",
                [DeckNode::deck("Vocab", [
                    Card::new("manger", "to eat"),
                    Card::new("voir", "to see"),
                    Card::new("faire", "to do"),
                    Card::new("gravir", "to climb"),
                ]), DeckNode::deck("Nouns", [])],
            ),
            DeckNode::deck("Comp Sci", []),
        ],
    );

    // Prompt the user with the options in the main menu
    // Since this returns an `InquireResult<()>`, and a Result must be handled (it could
    // potentially be an error!), we must assign it to *something*.
    // We don't care about if we have an error, so just assign it to nothing (`_`).
    let _ = prompt_main(&mut root);
}

/// Shows the main menu to the user. This uses the [inquire](https://docs.rs/inquire/) library,
/// allowing accepting validated user input easily.
fn prompt_main(root: &mut DeckNode) -> InquireResult<()> {
    enum Option {
        Deck { opt: NodePromptOption, index: usize },
        Stats,
        Quit,
    }

    impl fmt::Display for Option {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Option::Deck { opt, .. } => opt.fmt(f),
                Option::Stats => write!(f, "{}", STATS),
                Option::Quit => write!(f, "{}", QUIT),
            }
        }
    }

    let mut last_cursor = 0;
    loop {
        let mut options: Vec<Option> = root
            .prompt_options()
            .into_iter()
            .enumerate()
            .map(|(index, opt)| Option::Deck { opt, index })
            .collect();
        options.push(Option::Stats);
        options.push(Option::Quit);

        match Select::new(MAIN_MENU, options)
            .with_starting_cursor(last_cursor)
            .prompt()?
        {
            Option::Deck { opt, index } => {
                let Some(target) = root.at_mut(opt.path) else {
                    panic!("invalid target");
                };

                target.prompt_select(opt.action)?;
                last_cursor = index;
            }
            Option::Stats => return Ok(()),
            Option::Quit => {
                if prompt_confirm()? {
                    return Ok(());
                }
            }
        }
    }
}
