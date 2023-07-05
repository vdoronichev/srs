use inquire::Select;
use srs::prelude::*;

fn main() {
    let mut root = DeckNode::set(
        "All Decks",
        [
            DeckNode::set(
                "French",
                [
                    DeckNode::deck("Vocab", []),
                    DeckNode::deck("Nouns", []),
                ],
            ),
            DeckNode::deck("Comp Sci", []),
        ],
    );

    let _ = prompt_main(&mut root);
}

fn prompt_main(root: &mut DeckNode) -> InquireResult<()> {
    enum Option {
        Deck {
            opt: NodePromptOption,
            index: usize,
        },
        Stats,
        Quit,
    }

    impl std::fmt::Display for Option {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Option::Deck { opt, .. } => opt.fmt(f),
                Option::Stats => write!(f, "Stats"),
                Option::Quit => write!(f, "Quit"),
            }
        }
    }
    
    let mut last_cursor = 0;
    loop {
        let mut options: Vec<Option> = root
            .prompt_options()
            .into_iter()
            .enumerate()
            .map(|(index, opt)| Option::Deck {
                opt,
                index,
            })
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

                target.prompt_edit(opt.action)?;
                last_cursor = index;
            }
            Option::Stats => return Ok(()),
            Option::Quit => return Ok(()),
        }
    }
}
