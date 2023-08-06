use fltk::{dialog::*, enums::*};

pub fn main_help() {
    let mut help = HelpDialog::new(0,0, 1600, 900);
    help.load("/resources/help/main.html");
    help.show();
}