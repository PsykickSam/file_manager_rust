mod fd_struct;
mod constant;
mod util;
mod app;

#[macro_use]
extern crate crossterm;

fn main() {
    app::file_manager();    
}