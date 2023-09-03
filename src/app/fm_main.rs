use std::fs;
use std::io::Stdout;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyEvent, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use crate::fd_struct::FileDetails;
use crate::constant::{INIT_PATH, EXIT_TEXT};
use crate::util::{display_text, filter};

#[derive(Debug)]
pub struct FileManger<'a> {
    is_printed: bool,
    is_section_print: bool,
    point: u16,
    prev_point: u16,
    total_dir_files: u16,
    init_point: u16, 
    last_point: u16,
    cur_dir_stack: Vec<String>,
    new_dir_stack: Vec<String>,
    file_details: Vec<FileDetails>,
    is_stop: bool,
    stdout: &'a mut Stdout,
}

impl<'a> FileManger<'a> {
    pub fn init(stdout: &'a mut Stdout) -> FileManger {
        FileManger { 
            stdout,
            is_printed: false,
            is_section_print: false,
            point: 1,
            prev_point: 1,
            total_dir_files: 0,
            init_point: 0,
            last_point: 0,
            cur_dir_stack: Vec::new(),
            new_dir_stack: Vec::new(),
            file_details: Vec::new(),
            is_stop: false,
        }
    }

    pub fn setup(&'a mut self) -> &mut FileManger {
        self.cur_dir_stack.push(INIT_PATH.to_string());
        self.new_dir_stack.push(INIT_PATH.to_string());    
        self
    }

    fn event_handler(&mut self) {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.is_printed = !self.is_printed;
                self.prev_point = self.point;
                self.point = if self.point == self.init_point { self.last_point } else { self.init_point };
            },
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.is_printed = !self.is_printed;
                if self.point != self.total_dir_files {
                    if self.prev_point == self.point {
                        self.point += 1;
                    } else {
                        self.prev_point = self.point;
                        self.point += 1;
                    }  
                } 
            },
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.is_printed = !self.is_printed;
                if self.point != 1 {
                    if self.prev_point == self.point {
                        self.point -= 1;   
                    } else {
                        self.prev_point = self.point;
                        self.point -= 1;
                    }
                }
            },
            Event::Key(KeyEvent { 
                code: KeyCode::Enter, 
                modifiers: KeyModifiers::NONE, 
                ..
            }) => {
                let fd = filter(self.file_details.clone(), self.point);
                let fd = fd.get(0).unwrap();

                if fd.is_back {
                    self.is_printed = !self.is_printed;
                    self.is_section_print = false;
                    self.new_dir_stack.pop();
                } else if fd.is_dir {
                    self.is_printed = !self.is_printed;
                    self.is_section_print = false;
                    self.new_dir_stack.push(fd.path.clone());
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                display_text(&mut self.stdout, EXIT_TEXT);
                self.stop();
            },
            _ => (),
        }
    }

    fn ui(&mut self) {
        if self.is_printed == false {
            if self.new_dir_stack.get(self.new_dir_stack.len() - 1).unwrap() == self.cur_dir_stack.get(self.cur_dir_stack.len() - 1).unwrap() 
               && self.is_section_print {

                let prev_fd = filter(self.file_details.clone(), self.prev_point);
                let curr_fd = filter(self.file_details.clone(), self.point);

                let prev_fd = prev_fd.get(0).unwrap();
                let curr_fd = curr_fd.get(0).unwrap();

                execute!(self.stdout, cursor::MoveTo(0, prev_fd.index), Clear(ClearType::CurrentLine)).unwrap();
                if prev_fd.is_back {
                    execute!(self.stdout, cursor::Hide, Print(format!("{} {} {}", " ", prev_fd.name.clone(), ""))).unwrap();
                } else {
                    execute!(self.stdout, cursor::Hide, Print(format!("{} {} {}", " ", prev_fd.path.clone(), if prev_fd.is_file { format!("|> {}", prev_fd.name.clone()) } else { format!("") }))).unwrap();
                }
 
                execute!(self.stdout, cursor::MoveTo(0, curr_fd.index), Clear(ClearType::CurrentLine)).unwrap();
                if curr_fd.is_back {
                    execute!(self.stdout, cursor::Hide, Print(format!("{} {} {}", ">", curr_fd.name.clone(), format!("| {}", curr_fd.path)))).unwrap();
                } else {
                    execute!(self.stdout, cursor::Hide, Print(format!("{} {} {}", ">", curr_fd.path.clone(), if curr_fd.is_file { format!("|> {}", curr_fd.name.clone()) } else { format!("") }))).unwrap();
                }
            } else {
                // RESET
                self.point = if self.new_dir_stack.len() == 1 { 1 } else { 2 };
                self.prev_point = 1;
                self.is_section_print = true;
                self.cur_dir_stack.push(self.new_dir_stack.get(self.new_dir_stack.len() - 1).unwrap().clone());
                
                let mut i = 0;
                let read_dir = fs::read_dir(self.cur_dir_stack.get(self.cur_dir_stack.len() - 1).unwrap().clone()).unwrap();

                // CLEAR
                self.file_details.clear();

                execute!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, i)).unwrap();
                execute!(self.stdout, Print("--- SHOWING DISKS ---")).unwrap();
                i += 1; 

                if self.new_dir_stack.len() > 1 {
                    self.file_details.push(FileDetails::newb(i, "<".to_string(), self.new_dir_stack.get(self.new_dir_stack.len() - 2).unwrap().to_string(), false, true, true));
                    execute!(self.stdout, cursor::MoveTo(0, i)).unwrap();
                    execute!(self.stdout, Print(format!("{} <", " "))).unwrap();
                    i += 1;     
                }

                for item in read_dir {
                    let file_name = item.as_ref().unwrap().file_name().to_str().unwrap().to_string();
                    let path = item.as_ref().unwrap().path().display().to_string();
                    let is_file = item.as_ref().unwrap().metadata().unwrap().is_file();
                    let is_dir = item.as_ref().unwrap().metadata().unwrap().is_dir();
                    let dir_path = path.to_string();
                    
                    self.file_details.push(FileDetails::new(i, file_name.clone(), dir_path.clone(), is_file, is_dir));
    
                    execute!(self.stdout, cursor::MoveTo(0, i)).unwrap();
                    execute!(self.stdout, cursor::Hide, Print(
                        format!(
                            "{} {} {}", 
                            if self.point == i { ">" } else { " " }, 
                            dir_path.clone(), 
                            if is_file { format!("|> {}", file_name.clone()) } else { format!("") },
                        ))).unwrap();
            
                    i += 1;
                }

                // RESET
                if i == 2 { // IF NOTHING FOUND INSIDE FOLDER
                    let curr_fd = self.file_details.get(self.file_details.len() - 1).unwrap();
                    execute!(self.stdout, cursor::MoveTo(0, curr_fd.index), Clear(ClearType::CurrentLine)).unwrap();
                    execute!(self.stdout, Print(format!("{} < {}", ">", format!("| {}", curr_fd.path)))).unwrap();
                    self.point = 1;
                }

                self.init_point = self.point;
                self.last_point = i - 1;
                self.total_dir_files = self.last_point;
            }

            self.is_printed = !self.is_printed;
        }
    }

    fn stop(&mut self) {
        self.is_stop = true;
    }

    pub fn run(&mut self) {
        loop {
            if self.is_stop { 
                break; 
            }

            // UI
            self.ui();

            // EVENT HANDLER 
            self.event_handler();
        }
    }    
}

