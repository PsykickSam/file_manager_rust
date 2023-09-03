use std::io::Stdout;

use crossterm::{terminal::{Clear, ClearType}, cursor, style::Print};

use crate::fd_struct::FileDetails;

// FILER DIR VEC DATA BY POINT 
pub fn filter(list: Vec<FileDetails>, point: u16) -> Vec<FileDetails> {
    list.into_iter().filter(|p| {
        p.index == point
    }).collect()
}

pub fn display_text(stdout: &mut Stdout, text: &str) {
    let split_text = text.split("\n");
    let mut text_index = 0;
    
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), cursor::Hide).unwrap();
    split_text.for_each(|text| {
        execute!(stdout, cursor::MoveTo(0, text_index), cursor::Hide).unwrap();
        execute!(stdout, Print(format!("{}", text))).unwrap();
        text_index += 1;
    });
}