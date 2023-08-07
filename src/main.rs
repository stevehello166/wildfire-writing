// the Import block
mod file_manipulation;
use file_manipulation::*;

use fltk::{
    app,
    prelude::*,
    window::Window, 
    menu, menu::*, 
    enums::*,
    button::Button,
    dialog::HelpDialog,
    group,
    frame,
    button,
    dialog
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
            "Help" => {
                let mut help = HelpDialog::new(0,0, 1600, 900);
                help.load("/resources/help/main.html");
                help.show();
            },
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
    dialog::message_title_default("Wildfire Write");
    // all the widgets for the window
    let mut wind = Window::new(720, 1280, 1600, 900, APP_NAME);
    let mut menubar = SysMenuBar::new(0,0,1600, 25, APP_NAME);

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
    menubar.add(
        "Help\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        |_| {
            let mut help = HelpDialog::new(0,0, 1600, 900);
            help.load("resources/help/main.html");
            help.show();

            while help.shown(){
                app::wait();
            }
        }
    );


    let mut side_menu = Window::new(0, 25, 256, 875, "");
    side_menu.set_color(Color::Black);
    let mut side_menu_opt = group::Pack::default_fill();
    side_menu_opt.set_spacing(5);

    for i in 0..2 {
        //variables for creating the side menu
        let module_num: String = i.to_string();
        let module_num: String = "module".to_string() + module_num.as_str();
        let module_name: [&str; 2] = ["Character Module", "Timeline Module"];

        //Creates the buttons
        button::Button::default().with_size(0, 80).with_label(module_name[i]).set_id(&module_num);
    }
    side_menu.end();
    
    //The module window
    let mut module_menu = Window::new(256,25, 1344, 875, "");

    module_menu.end();

    wind.end();
    wind.show();
    a.run().unwrap();
    //Runs at application close
}