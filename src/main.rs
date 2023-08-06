// the Import block
mod file_manipulation;
use file_manipulation::*;

use fltk::{
    app,
    prelude::*,
    window::Window, 
    menu, menu::*, 
    enums::*, 
    group::{FlexType::Row, ScrollType},
    button::Button
    };

use std::env;

pub const OPERATING_SYSTEM: &str = env::consts::OS;
pub const APP_NAME: &str = "Wildfire Write";

fn menu_cb(m: &mut impl MenuExt){
    let mut file_str:String;
    file_str = "".to_string();
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "New Note\t" => create_file(".md"),
            "Open\t" =>  file_str = open_file(),
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
    println!("{}", OPERATING_SYSTEM);
    let a = app::App::default();
    // all the widgets for the window
    let mut wind = Window::new(720, 1280, 400, 300, APP_NAME);
    let mut menubar = SysMenuBar::new(0,0,400, 25, APP_NAME);

    let open_file_str: String;
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

    let mut divider = Row;
    let mut scrolled_menu = ScrollType::VerticalAlways;
    //let mut module1 = Button::new(center_of(scrolled_menu), center_of(scrolled_menu),5);
    wind.end();
    wind.show();
    a.run().unwrap();
}


