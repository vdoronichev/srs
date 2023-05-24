use srs::prelude::*;

fn main() {
    let mut deck_set = DeckSet::new("<root>");

    let _ = deck_set.prompt_edit();
}
