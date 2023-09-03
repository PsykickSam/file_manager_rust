use std::io::stdout;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, event::{read, Event, KeyEvent, KeyCode, KeyModifiers}};

use crate::{util::display_text, constant::{INIT_TEXT, EXIT_TEXT}};

use super::fm_main::FileManger;

fn input_taker(is_exit: &mut bool) {
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                *is_exit = false;
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                *is_exit = true;
                break;
            },
            _ => ()
        }
    }
}

pub fn file_manager() {
    let mut stdout = stdout();
    let mut is_exit = false;

    // ENABLE RAW MODE 
    enable_raw_mode().unwrap();

    // CLEAR AND MOVE THE THE CURSOR TO TOP LEFT WITH MESSAGE 
    display_text(&mut stdout, INIT_TEXT);

    // INPUT
    input_taker(&mut is_exit);

    // MAIN 
    if is_exit {
        display_text(&mut stdout, EXIT_TEXT);
    } else {
        FileManger::init(&mut stdout).setup().run();
    }
 
    // DISABLE RAW MODE
    disable_raw_mode().unwrap();
}
