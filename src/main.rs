// the Import block
mod file_manipulation;
use file_manipulation::create_file;

use fltk::{
    app,
    prelude::*,
    window::Window, 
    menu, menu::*, 
    enums::*, 
    group::FlexType::Column
    };

use std::env;

pub const OPERATING_SYSTEM: &str = env::consts::OS;

fn menu_cb(m: &mut impl MenuExt) {
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "New Note\t" => create_file(".md"),
            "Open\t" => println!("Open"),
            "Third" => println!("Third"),
            "Quit\t" => {
                println!("Quitting");
                app::quit();
            },
            _ => println!("{}", choice),
        }
    }
}

fn main() {
    let a = app::App::default();
    // all the widgets for the window
    let mut wind = Window::new(720, 1280, 400, 300, "Libre Writer");
    let mut menubar = SysMenuBar::new(0,0,400, 25, "Libre Writer");

    // menu bar stuff
    menubar.add(
        "File/New Note\t",
        Shortcut::None,
        menu::MenuFlag::Normal, 
        menu_cb);
    menubar.add(
        "File/Open\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    menubar.add(
        "File/Quit\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    menubar.add(
       "Edit/Insert\t", 
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb
    );

    let mut divider = Column;
    wind.end();
    wind.show();
    a.run().unwrap();
}





