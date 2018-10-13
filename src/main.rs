extern crate ncurses;

use ncurses::*;

mod text;

fn main() {
    let _text = text::get_text();
    initscr();
    noecho();
    for c in _text.chars() {
        getch();
        printw(c.to_string().as_str());
    }
}
