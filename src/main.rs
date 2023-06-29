use inquire::Select;
use srs::prelude::*;

fn prompt_main(root: &mut DeckNode) -> InquireResult<()> {
    loop {

    }
}

fn main() {
    let root = DeckNode::set(
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
}
