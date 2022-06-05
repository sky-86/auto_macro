use inputbot::{KeySequence, KeybdKey::*};
use std::process::exit;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// fn read_args() -> bool {
//     true
// }

// fn ask_for_user() -> bool {
//     true
// }

#[derive(Serialize, Deserialize)]
struct Account {
    alias: String,
    username: String,
    password: String,
}

fn read_json() {

}

// fn create_json() -> bool {
//     true
// }

// fn add_user() -> bool {
//     true
// }

fn bind_macro(user: &'static str, pass: &'static str) {
    F1Key.bind(move || {
        KeySequence(user).send();
    });

    F2Key.bind(move || {
        KeySequence(pass).send();
        exit(0);
    });

    inputbot::handle_input_events();
}

fn main() {
    bind_macro("bob123", "1234!");
}