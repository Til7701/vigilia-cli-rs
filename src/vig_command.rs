use crate::{index, query, ui};
use std::process::exit;

pub fn dialog() {
    let selected = ui::select(
        "Select an action:",
        vec![
            "Index new files".to_string(),
            "Query indexed files".to_string(),
            "Exit".to_string(),
        ],
    );
    match selected {
        0 => index::dialog(),
        1 => query::dialog(),
        2 => exit(0),
        _ => unreachable!(),
    }
}
